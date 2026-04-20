//! Combat system for NovaForge.
//!
//! Handles damage application, weapon types, and NPC pirate behaviour.
//! Damage is split into four types (EM, Thermal, Kinetic, Explosive) and
//! is reduced by the target's resistance profile for the relevant tank layer.

use fyrox::core::reflect::prelude::*;
use fyrox::core::visitor::prelude::*;

use crate::ship::ResistanceProfile;

/// The four damage types used throughout NovaForge.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Visit, Reflect)]
pub enum DamageType {
    EM,
    Thermal,
    Kinetic,
    Explosive,
}

impl Default for DamageType {
    fn default() -> Self {
        DamageType::Kinetic
    }
}

/// A packet of damage delivered to a target.
#[derive(Debug, Clone, Default, Visit, Reflect)]
pub struct DamagePacket {
    pub em: f32,
    pub thermal: f32,
    pub kinetic: f32,
    pub explosive: f32,
}

impl DamagePacket {
    /// Total raw damage before resistances.
    pub fn total(&self) -> f32 {
        self.em + self.thermal + self.kinetic + self.explosive
    }

    /// Applies a resistance profile and returns the actual damage taken.
    pub fn apply_resists(&self, resists: &ResistanceProfile) -> f32 {
        self.em * (1.0 - resists.em)
            + self.thermal * (1.0 - resists.thermal)
            + self.kinetic * (1.0 - resists.kinetic)
            + self.explosive * (1.0 - resists.explosive)
    }
}

/// Weapon category.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Visit, Reflect)]
pub enum WeaponType {
    /// Turret weapons (tracking-based hit chance).
    Turret(TurretKind),
    /// Missile weapons (always hit, damage scaled by signature/explosion radius).
    Missile(MissileKind),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Visit, Reflect, Default)]
pub enum TurretKind {
    Projectile,
    #[default]
    Hybrid,
    Energy,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Visit, Reflect, Default)]
pub enum MissileKind {
    #[default]
    Rocket,
    LightMissile,
    HeavyMissile,
    CruiseMissile,
    Torpedo,
}

/// Runtime state for a fitted weapon module.
#[derive(Debug, Clone, Visit, Reflect)]
pub struct Weapon {
    pub weapon_type: WeaponType,
    /// Base damage packet per shot / volley.
    pub base_damage: DamagePacket,
    /// Rate of fire in shots per second.
    pub rate_of_fire: f32,
    /// Optimal range in metres.
    pub optimal_range: f32,
    /// Falloff / flight-time degradation range in metres.
    pub falloff: f32,
    /// Turret tracking speed (rad/s). Ignored for missiles.
    pub tracking_speed: f32,
    /// Seconds until next shot is ready.
    pub reload_timer: f32,
}

impl Default for Weapon {
    fn default() -> Self {
        Weapon {
            weapon_type: WeaponType::Turret(TurretKind::Hybrid),
            base_damage: DamagePacket { kinetic: 24.0, thermal: 16.0, ..Default::default() },
            rate_of_fire: 1.0,
            optimal_range: 5_000.0,
            falloff: 2_500.0,
            tracking_speed: 0.05,
            reload_timer: 0.0,
        }
    }
}

impl Weapon {
    /// Advances the reload timer by `dt` seconds. Returns `true` when ready to fire.
    pub fn tick(&mut self, dt: f32) -> bool {
        if self.reload_timer > 0.0 {
            self.reload_timer = (self.reload_timer - dt).max(0.0);
        }
        self.reload_timer == 0.0
    }

    /// Fires the weapon, resetting the reload timer. Returns the damage packet.
    pub fn fire(&mut self) -> DamagePacket {
        self.reload_timer = 1.0 / self.rate_of_fire;
        self.base_damage.clone()
    }

    /// Hit-chance for a turret based on distance and target angular velocity.
    /// Returns a value in [0.0, 1.0].
    pub fn hit_chance_turret(&self, distance: f32, angular_velocity: f32) -> f32 {
        let range_factor = if distance <= self.optimal_range {
            1.0
        } else {
            let excess = distance - self.optimal_range;
            (-excess * excess / (2.0 * self.falloff * self.falloff)).exp()
        };
        let tracking_factor = if angular_velocity < f32::EPSILON {
            1.0
        } else {
            (self.tracking_speed / angular_velocity).min(1.0)
        };
        range_factor * tracking_factor
    }
}

/// Live hit-point state for a ship in combat.
#[derive(Debug, Clone, Visit, Reflect)]
pub struct HitPoints {
    pub shield_current: f32,
    pub shield_max: f32,
    pub armor_current: f32,
    pub armor_max: f32,
    pub hull_current: f32,
    pub hull_max: f32,

    pub shield_regen_rate: f32,
}

impl Default for HitPoints {
    fn default() -> Self {
        HitPoints {
            shield_current: 600.0,
            shield_max: 600.0,
            armor_current: 450.0,
            armor_max: 450.0,
            hull_current: 450.0,
            hull_max: 450.0,
            shield_regen_rate: 5.0,
        }
    }
}

impl HitPoints {
    /// Applies a damage packet, draining shield → armor → hull in order.
    /// Returns `true` if the ship has been destroyed.
    pub fn apply_damage(
        &mut self,
        damage: &DamagePacket,
        shield_resists: &ResistanceProfile,
        armor_resists: &ResistanceProfile,
        hull_resists: &ResistanceProfile,
    ) -> bool {
        let mut remaining = damage.apply_resists(shield_resists);

        // Hit shield first
        let shield_taken = remaining.min(self.shield_current);
        self.shield_current -= shield_taken;
        remaining -= shield_taken;

        if remaining > 0.0 {
            let armor_taken = remaining.min(self.armor_current);
            let armor_damage = DamagePacket {
                em: damage.em * armor_taken / damage.total().max(1.0),
                thermal: damage.thermal * armor_taken / damage.total().max(1.0),
                kinetic: damage.kinetic * armor_taken / damage.total().max(1.0),
                explosive: damage.explosive * armor_taken / damage.total().max(1.0),
            };
            let actual_armor_damage = armor_damage.apply_resists(armor_resists);
            self.armor_current = (self.armor_current - actual_armor_damage).max(0.0);
            remaining -= armor_taken;
        }

        if remaining > 0.0 {
            let hull_damage = DamagePacket {
                em: damage.em * remaining / damage.total().max(1.0),
                thermal: damage.thermal * remaining / damage.total().max(1.0),
                kinetic: damage.kinetic * remaining / damage.total().max(1.0),
                explosive: damage.explosive * remaining / damage.total().max(1.0),
            };
            let actual_hull_damage = hull_damage.apply_resists(hull_resists);
            self.hull_current = (self.hull_current - actual_hull_damage).max(0.0);
        }

        self.hull_current <= 0.0
    }

    /// Regenerates shield by `dt` seconds of passive recharge.
    pub fn regen_shield(&mut self, dt: f32) {
        if self.shield_current < self.shield_max {
            self.shield_current = (self.shield_current + self.shield_regen_rate * dt)
                .min(self.shield_max);
        }
    }

    /// Fraction of each layer remaining (0.0–1.0).
    pub fn shield_fraction(&self) -> f32 {
        self.shield_current / self.shield_max.max(1.0)
    }
    pub fn armor_fraction(&self) -> f32 {
        self.armor_current / self.armor_max.max(1.0)
    }
    pub fn hull_fraction(&self) -> f32 {
        self.hull_current / self.hull_max.max(1.0)
    }
}

/// NPC behaviour state for pirate ships.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Visit, Reflect)]
pub enum NpcState {
    Idle,
    Approaching,
    Orbiting,
    Fleeing,
    Dead,
}

impl Default for NpcState {
    fn default() -> Self {
        NpcState::Idle
    }
}
