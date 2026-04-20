//! Player character data for NovaForge.
//!
//! Aggregates character attributes, skill book, wallet, inventory, and the
//! currently active ship type into a single serialisable struct.

use fyrox::core::reflect::prelude::*;
use fyrox::core::visitor::prelude::*;

use crate::economy::{Inventory, Wallet};
use crate::faction::FactionStandings;
use crate::ship::Race;
use crate::skills::{CharacterAttributes, SkillBook};

/// The character's chosen race and bloodline.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Visit, Reflect, Default)]
pub enum Bloodline {
    // Amarr
    Amarr,
    NiKunni,
    Khanid,
    // Caldari
    #[default]
    Achura,
    Civire,
    Deteis,
    // Gallente
    Gallente,
    Intaki,
    JinMei,
    // Minmatar
    Brutor,
    Sebiestor,
    Vherokior,
}

/// A fully initialised player character.
#[derive(Debug, Clone, Visit, Reflect)]
pub struct Character {
    pub name: String,
    pub race: Race,
    pub bloodline: Bloodline,

    pub attributes: CharacterAttributes,
    pub skills: SkillBook,
    pub standings: FactionStandings,
    pub wallet: Wallet,
    pub inventory: Inventory,

    /// Ship type the character is currently flying.
    pub active_ship_type: String,
}

impl Default for Character {
    fn default() -> Self {
        let attributes = CharacterAttributes {
            perception: 20.0,
            memory: 20.0,
            willpower: 20.0,
            intelligence: 20.0,
            charisma: 19.0,
        };

        let mut skills = SkillBook::default();
        // All characters start with basic navigation and spaceship command.
        for starter in &[
            "Spaceship Command",
            "Navigation",
            "Gunnery",
            "Engineering",
            "Electronics",
            "Shield Operation",
            "Target Management",
        ] {
            skills.inject(starter);
        }
        // Start training the default skill.
        skills.start_training("Spaceship Command");

        Character {
            name: "New Capsuleer".into(),
            race: Race::Caldari,
            bloodline: Bloodline::Achura,
            attributes,
            skills,
            standings: FactionStandings::default(),
            wallet: Wallet { isk: 50_000.0 },
            inventory: Inventory::default(),
            active_ship_type: "Merlin".into(),
        }
    }
}
