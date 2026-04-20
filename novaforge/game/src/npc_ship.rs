//! NPC pirate ship script for NovaForge.
//!
//! Provides basic stationary-idle → approach → orbit combat AI for pirate
//! frigates and cruisers found in asteroid belts and anomalies.

#[allow(unused_imports)]
use fyrox::graph::prelude::*;
use fyrox::{
    core::{
        algebra::Vector3,
        reflect::prelude::*,
        type_traits::prelude::*,
        visitor::prelude::*,
    },
    event::Event,
    plugin::error::GameResult,
    script::{ScriptContext, ScriptDeinitContext, ScriptTrait},
};

use crate::combat::{HitPoints, NpcState, Weapon};
use crate::ship::ShipStats;

/// Preferred combat range in metres.
const ORBIT_RANGE: f32 = 5_000.0;
/// Distance at which the NPC starts approaching the player.
const AGGRO_RANGE: f32 = 30_000.0;

/// Script for NPC pirate ships.
#[derive(Visit, Reflect, Default, Debug, Clone, TypeUuidProvider, ComponentProvider)]
#[type_uuid(id = "b2c3d4e5-f6a7-8901-bcde-f23456789012")]
#[visit(optional)]
pub struct NpcShip {
    pub stats: ShipStats,
    pub hit_points: HitPoints,
    pub weapon: Weapon,
    pub state: NpcState,
    /// ISK bounty awarded when this NPC is destroyed.
    pub bounty: u64,

    /// Handle to the player ship node (resolved at start).
    player_handle: Option<fyrox::core::pool::Handle<fyrox::scene::node::Node>>,
    weapon_timer: f32,
}

impl ScriptTrait for NpcShip {
    fn on_init(&mut self, _context: &mut ScriptContext) -> GameResult {
        self.hit_points = HitPoints {
            shield_current: self.stats.shield_hp,
            shield_max: self.stats.shield_hp,
            armor_current: self.stats.armor_hp,
            armor_max: self.stats.armor_hp,
            hull_current: self.stats.hull_hp,
            hull_max: self.stats.hull_hp,
            shield_regen_rate: self.stats.shield_hp / 180.0,
        };
        self.state = NpcState::Idle;
        Ok(())
    }

    fn on_start(&mut self, context: &mut ScriptContext) -> GameResult {
        // Find the player ship node by tag.
        for (handle, node) in context.scene.graph.pair_iter() {
            if node.tag() == "PlayerShip" {
                self.player_handle = Some(handle);
                break;
            }
        }
        Ok(())
    }

    fn on_deinit(&mut self, _context: &mut ScriptDeinitContext) -> GameResult {
        Ok(())
    }

    fn on_os_event(&mut self, _event: &Event<()>, _context: &mut ScriptContext) -> GameResult {
        Ok(())
    }

    fn on_update(&mut self, context: &mut ScriptContext) -> GameResult {
        if self.state == NpcState::Dead {
            return Ok(());
        }

        let dt = context.dt;
        self.hit_points.regen_shield(dt);
        self.weapon_timer = (self.weapon_timer - dt).max(0.0);

        let my_pos = **context.scene.graph[context.handle]
            .local_transform()
            .position();

        let Some(player_handle) = self.player_handle else {
            return Ok(());
        };

        let player_pos = **context.scene.graph[player_handle]
            .local_transform()
            .position();

        let to_player = player_pos - my_pos;
        let distance = to_player.norm();

        match self.state {
            NpcState::Idle => {
                if distance < AGGRO_RANGE {
                    self.state = NpcState::Approaching;
                }
            }
            NpcState::Approaching => {
                if distance <= ORBIT_RANGE {
                    self.state = NpcState::Orbiting;
                } else {
                    // Move toward player.
                    let dir = to_player / distance;
                    let new_pos = my_pos + dir * self.stats.max_velocity * dt;
                    context.scene.graph[context.handle]
                        .set_position(new_pos);
                }
            }
            NpcState::Orbiting => {
                // Simple circular orbit: strafe perpendicular to the player vector.
                let perp = Vector3::new(-to_player.z, 0.0, to_player.x).normalize();
                let orbit_pos = player_pos + to_player.normalize() * ORBIT_RANGE;
                let move_dir = (orbit_pos - my_pos).normalize();
                let new_pos =
                    my_pos + (move_dir + perp * 0.3).normalize() * self.stats.max_velocity * dt;
                context.scene.graph[context.handle]
                        .set_position(new_pos);

                // Fire if weapon is ready.
                if self.weapon_timer == 0.0 && distance <= self.weapon.optimal_range * 2.0 {
                    let _damage = self.weapon.fire();
                    self.weapon_timer = 1.0 / self.weapon.rate_of_fire;
                    // Damage delivery to player handled by the combat manager in Game::update.
                }
            }
            NpcState::Fleeing => {
                // Move away from player.
                if distance > 0.0 {
                    let dir = -to_player / distance;
                    let new_pos = my_pos + dir * self.stats.max_velocity * dt;
                    context.scene.graph[context.handle]
                        .set_position(new_pos);
                }
            }
            NpcState::Dead => {}
        }

        Ok(())
    }
}
