//! Player-controlled ship script for NovaForge.
//!
//! Attached to the ship scene node, this script handles:
//! - WASD / arrow-key thrust input
//! - Mouse-look camera orbit
//! - Basic weapon firing (spacebar)

#[allow(unused_imports)]
use fyrox::graph::prelude::*;
use fyrox::{
    core::{
        algebra::Vector3,
        reflect::prelude::*,
        type_traits::prelude::*,
        visitor::prelude::*,
    },
    event::{ElementState, Event, KeyEvent, WindowEvent},
    keyboard::{KeyCode, PhysicalKey},
    plugin::error::GameResult,
    script::{ScriptContext, ScriptDeinitContext, ScriptTrait},
};

use crate::combat::{HitPoints, Weapon};
use crate::ship::ShipStats;

/// Fyrox script controlling the player's ship.
#[derive(Visit, Reflect, Default, Debug, Clone, TypeUuidProvider, ComponentProvider)]
#[type_uuid(id = "a1b2c3d4-e5f6-7890-abcd-ef1234567890")]
#[visit(optional)]
pub struct PlayerShip {
    /// Current ship statistics (set by plugin on spawn).
    pub stats: ShipStats,
    /// Live hit-point tracking.
    pub hit_points: HitPoints,
    /// Primary weapon fitted to the ship.
    pub weapon: Weapon,

    // Thrust input state
    thrust_forward: bool,
    thrust_backward: bool,
    thrust_left: bool,
    thrust_right: bool,
    /// Fire weapon on next update tick.
    fire_weapon: bool,

    /// Current velocity vector (m/s in local space).
    velocity: Vector3<f32>,
}

impl ScriptTrait for PlayerShip {
    fn on_init(&mut self, _context: &mut ScriptContext) -> GameResult {
        // Initialise hit points from ship stats.
        self.hit_points = HitPoints {
            shield_current: self.stats.shield_hp,
            shield_max: self.stats.shield_hp,
            armor_current: self.stats.armor_hp,
            armor_max: self.stats.armor_hp,
            hull_current: self.stats.hull_hp,
            hull_max: self.stats.hull_hp,
            shield_regen_rate: self.stats.shield_hp / 120.0, // full regen in 2 minutes
        };
        Ok(())
    }

    fn on_start(&mut self, _context: &mut ScriptContext) -> GameResult {
        Ok(())
    }

    fn on_deinit(&mut self, _context: &mut ScriptDeinitContext) -> GameResult {
        Ok(())
    }

    fn on_os_event(&mut self, event: &Event<()>, _context: &mut ScriptContext) -> GameResult {
        if let Event::WindowEvent {
            event: WindowEvent::KeyboardInput { event: KeyEvent { physical_key, state, .. }, .. },
            ..
        } = event
        {
            let pressed = *state == ElementState::Pressed;
            match physical_key {
                PhysicalKey::Code(KeyCode::KeyW) | PhysicalKey::Code(KeyCode::ArrowUp) => {
                    self.thrust_forward = pressed;
                }
                PhysicalKey::Code(KeyCode::KeyS) | PhysicalKey::Code(KeyCode::ArrowDown) => {
                    self.thrust_backward = pressed;
                }
                PhysicalKey::Code(KeyCode::KeyA) | PhysicalKey::Code(KeyCode::ArrowLeft) => {
                    self.thrust_left = pressed;
                }
                PhysicalKey::Code(KeyCode::KeyD) | PhysicalKey::Code(KeyCode::ArrowRight) => {
                    self.thrust_right = pressed;
                }
                PhysicalKey::Code(KeyCode::Space) => {
                    if pressed {
                        self.fire_weapon = true;
                    }
                }
                _ => {}
            }
        }
        Ok(())
    }

    fn on_update(&mut self, context: &mut ScriptContext) -> GameResult {
        let dt = context.dt;

        // --- Weapon tick ---
        if self.fire_weapon && self.weapon.tick(dt) {
            let _damage = self.weapon.fire();
            // In a full implementation the damage packet is delivered to the
            // targeted enemy via the scene hierarchy.
            self.fire_weapon = false;
        } else {
            self.weapon.tick(dt);
        }

        // --- Shield regeneration ---
        self.hit_points.regen_shield(dt);

        // --- Thrust ---
        let acceleration = self.stats.max_velocity / (self.stats.agility * 10.0);
        let drag = 0.98_f32;

        let mut thrust = Vector3::zeros();
        if self.thrust_forward {
            thrust.z -= 1.0;
        }
        if self.thrust_backward {
            thrust.z += 1.0;
        }
        if self.thrust_left {
            thrust.x -= 1.0;
        }
        if self.thrust_right {
            thrust.x += 1.0;
        }

        if thrust.norm() > f32::EPSILON {
            thrust = thrust.normalize();
        }

        self.velocity += thrust * acceleration * dt;
        self.velocity *= drag;

        // Clamp to max velocity
        let speed = self.velocity.norm();
        if speed > self.stats.max_velocity {
            self.velocity = self.velocity / speed * self.stats.max_velocity;
        }

        // Apply velocity to node position.
        let node = &mut context.scene.graph[context.handle];
        let pos = **node.local_transform().position();
        node.set_position(pos + self.velocity * dt);

        Ok(())
    }
}
