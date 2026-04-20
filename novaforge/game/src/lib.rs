//! NovaForge — a PvE-focused sci-fi space MMO built on the Fyrox engine.
//!
//! This crate is the game plugin. It wires together all NovaForge systems
//! (ships, factions, skills, combat, economy) and registers every script with
//! the engine.

#[allow(unused_imports)]
use fyrox::graph::prelude::*;
use fyrox::{
    core::{pool::Handle, reflect::prelude::*, visitor::prelude::*},
    event::Event,
    gui::{message::UiMessage, UserInterface},
    plugin::{error::GameResult, Plugin, PluginContext, PluginRegistrationContext},
};

pub mod character;
pub mod combat;
pub mod economy;
pub mod faction;
pub mod npc_ship;
pub mod player_ship;
pub mod ship;
pub mod skills;

use character::Character;
use economy::OrderBook;

// Re-export the engine so executors do not need to depend on it separately.
pub use fyrox;

/// Top-level game state.
#[derive(Default, Visit, Reflect, Debug)]
#[reflect(non_cloneable)]
pub struct Game {
    /// The active player character. Persists across scenes (in-space / docked).
    pub character: Character,
    /// Station-local market order book.
    pub market: OrderBook,
}

impl Plugin for Game {
    fn register(&self, context: PluginRegistrationContext) -> GameResult {
        context
            .serialization_context
            .script_constructors
            .add::<player_ship::PlayerShip>("Player Ship");
        context
            .serialization_context
            .script_constructors
            .add::<npc_ship::NpcShip>("NPC Ship");
        Ok(())
    }

    fn init(&mut self, scene_path: Option<&str>, mut context: PluginContext) -> GameResult {
        // Load the default space scene. The scene file is expected at
        // `data/scene.rgs` relative to the working directory.
        context.load_scene_or_ui::<Self>(scene_path.unwrap_or("data/scene.rgs"));
        Ok(())
    }

    fn on_deinit(&mut self, _context: PluginContext) -> GameResult {
        Ok(())
    }

    fn update(&mut self, context: &mut PluginContext) -> GameResult {
        let dt = context.dt;

        // Advance passive skill training every frame.
        if let Some(levelled_skill) = self
            .character
            .skills
            .tick(dt, &self.character.attributes)
        {
            fyrox::core::log::Log::info(format!(
                "Skill levelled up: {}  (new level: {})",
                levelled_skill,
                self.character.skills.level(&levelled_skill),
            ));
        }

        Ok(())
    }

    fn on_os_event(&mut self, _event: &Event<()>, _context: PluginContext) -> GameResult {
        Ok(())
    }

    fn on_ui_message(
        &mut self,
        _context: &mut PluginContext,
        _message: &UiMessage,
        _ui_handle: Handle<UserInterface>,
    ) -> GameResult {
        Ok(())
    }
}
