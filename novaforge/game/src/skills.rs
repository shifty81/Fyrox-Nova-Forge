//! Skill system for NovaForge.
//!
//! Skills train in real-time (wall-clock seconds), continuing even while the
//! game is not running. Each skill has a rank (training time multiplier) and
//! a primary/secondary attribute pair that determines training speed.

use fyrox::core::reflect::prelude::*;
use fyrox::core::visitor::prelude::*;
use std::collections::HashMap;

/// Character attributes that govern skill training speed.
#[derive(Debug, Clone, Default, Visit, Reflect)]
pub struct CharacterAttributes {
    pub perception: f32,
    pub memory: f32,
    pub willpower: f32,
    pub intelligence: f32,
    pub charisma: f32,
}

impl CharacterAttributes {
    /// Returns the training speed multiplier for a skill with the given
    /// primary and secondary attribute names.
    pub fn training_speed(&self, primary: Attribute, secondary: Attribute) -> f32 {
        self.value(primary) + self.value(secondary) / 2.0
    }

    fn value(&self, attr: Attribute) -> f32 {
        match attr {
            Attribute::Perception => self.perception,
            Attribute::Memory => self.memory,
            Attribute::Willpower => self.willpower,
            Attribute::Intelligence => self.intelligence,
            Attribute::Charisma => self.charisma,
        }
    }
}

/// Character attribute types.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Attribute {
    Perception,
    Memory,
    Willpower,
    Intelligence,
    Charisma,
}

/// A skill definition: what it does and how fast it trains.
#[derive(Debug, Clone)]
pub struct SkillDef {
    pub name: &'static str,
    pub description: &'static str,
    /// Training time multiplier (rank). Each SP per level = 250 * rank * 32^(level-1).
    pub rank: u32,
    pub primary: Attribute,
    pub secondary: Attribute,
}

/// A player's current level and accumulated skill points for a specific skill.
#[derive(Debug, Clone, Default, Visit, Reflect)]
pub struct SkillProgress {
    /// Current trained level (0 = untrained, 5 = max).
    pub level: u8,
    /// Accumulated skill points toward the *next* level.
    pub sp_toward_next: f32,
}

impl SkillProgress {
    /// Returns the SP required to reach the next level.
    pub fn sp_required_for_next(&self, rank: u32) -> f32 {
        if self.level >= 5 {
            return f32::INFINITY;
        }
        // EVE formula: 250 * rank * 32^(level) for level→level+1
        250.0 * rank as f32 * (32_f32.powi(self.level as i32))
    }

    /// Adds SP toward the next level. Returns the number of levels gained (0 or 1).
    pub fn add_sp(&mut self, sp: f32, rank: u32) -> u8 {
        if self.level >= 5 {
            return 0;
        }
        self.sp_toward_next += sp;
        let needed = self.sp_required_for_next(rank);
        if self.sp_toward_next >= needed {
            self.sp_toward_next -= needed;
            self.level += 1;
            1
        } else {
            0
        }
    }
}

/// All skill definitions available in NovaForge.
pub struct SkillDatabase;

impl SkillDatabase {
    /// Returns the definition for a named skill, or `None` if unknown.
    pub fn get(name: &str) -> Option<SkillDef> {
        SKILLS.iter().find(|s| s.name == name).cloned()
    }

    /// Returns an iterator over all skill definitions.
    pub fn all() -> impl Iterator<Item = &'static SkillDef> {
        SKILLS.iter()
    }
}

static SKILLS: &[SkillDef] = &[
    // --- Gunnery ---
    SkillDef {
        name: "Gunnery",
        description: "Basic turret operation. Improves rate of fire.",
        rank: 1,
        primary: Attribute::Perception,
        secondary: Attribute::Willpower,
    },
    SkillDef {
        name: "Small Hybrid Turret",
        description: "Operation of small hybrid turrets.",
        rank: 1,
        primary: Attribute::Perception,
        secondary: Attribute::Willpower,
    },
    SkillDef {
        name: "Small Projectile Turret",
        description: "Operation of small projectile turrets.",
        rank: 1,
        primary: Attribute::Perception,
        secondary: Attribute::Willpower,
    },
    SkillDef {
        name: "Small Energy Turret",
        description: "Operation of small energy turrets.",
        rank: 2,
        primary: Attribute::Perception,
        secondary: Attribute::Willpower,
    },
    // --- Missiles ---
    SkillDef {
        name: "Missile Launcher Operation",
        description: "Basic missile launcher operation.",
        rank: 1,
        primary: Attribute::Perception,
        secondary: Attribute::Willpower,
    },
    SkillDef {
        name: "Light Missiles",
        description: "Allows use of light missiles.",
        rank: 1,
        primary: Attribute::Perception,
        secondary: Attribute::Willpower,
    },
    // --- Spaceship Command ---
    SkillDef {
        name: "Spaceship Command",
        description: "General spaceship operation. Improves agility.",
        rank: 1,
        primary: Attribute::Perception,
        secondary: Attribute::Willpower,
    },
    SkillDef {
        name: "Caldari Frigate",
        description: "Proficiency with Caldari frigates.",
        rank: 2,
        primary: Attribute::Perception,
        secondary: Attribute::Willpower,
    },
    SkillDef {
        name: "Caldari Destroyer",
        description: "Proficiency with Caldari destroyers.",
        rank: 3,
        primary: Attribute::Perception,
        secondary: Attribute::Willpower,
    },
    SkillDef {
        name: "Caldari Cruiser",
        description: "Proficiency with Caldari cruisers.",
        rank: 5,
        primary: Attribute::Perception,
        secondary: Attribute::Willpower,
    },
    // --- Engineering ---
    SkillDef {
        name: "Engineering",
        description: "Increases available power grid.",
        rank: 1,
        primary: Attribute::Intelligence,
        secondary: Attribute::Memory,
    },
    SkillDef {
        name: "Electronics",
        description: "Increases available CPU.",
        rank: 1,
        primary: Attribute::Intelligence,
        secondary: Attribute::Memory,
    },
    SkillDef {
        name: "Shield Operation",
        description: "Improves shield recharge rate.",
        rank: 1,
        primary: Attribute::Intelligence,
        secondary: Attribute::Memory,
    },
    SkillDef {
        name: "Armor Layering",
        description: "Improves armor hit points.",
        rank: 2,
        primary: Attribute::Intelligence,
        secondary: Attribute::Memory,
    },
    // --- Navigation ---
    SkillDef {
        name: "Navigation",
        description: "Improves sub-warp speed.",
        rank: 1,
        primary: Attribute::Intelligence,
        secondary: Attribute::Perception,
    },
    SkillDef {
        name: "Afterburner",
        description: "Improves afterburner thrust.",
        rank: 1,
        primary: Attribute::Intelligence,
        secondary: Attribute::Perception,
    },
    SkillDef {
        name: "Warp Drive Operation",
        description: "Improves warp speed.",
        rank: 1,
        primary: Attribute::Intelligence,
        secondary: Attribute::Perception,
    },
    // --- Targeting ---
    SkillDef {
        name: "Target Management",
        description: "Increases number of locked targets.",
        rank: 1,
        primary: Attribute::Intelligence,
        secondary: Attribute::Memory,
    },
    SkillDef {
        name: "Long Range Targeting",
        description: "Improves targeting range.",
        rank: 2,
        primary: Attribute::Intelligence,
        secondary: Attribute::Memory,
    },
    // --- Trade ---
    SkillDef {
        name: "Trade",
        description: "Increases number of active market orders.",
        rank: 1,
        primary: Attribute::Willpower,
        secondary: Attribute::Charisma,
    },
    SkillDef {
        name: "Broker Relations",
        description: "Reduces market broker fee.",
        rank: 2,
        primary: Attribute::Willpower,
        secondary: Attribute::Charisma,
    },
    // --- Drones ---
    SkillDef {
        name: "Drones",
        description: "Allows control of combat drones.",
        rank: 1,
        primary: Attribute::Memory,
        secondary: Attribute::Perception,
    },
];

/// The full set of skill progress for a character.
#[derive(Debug, Clone, Default, Visit, Reflect)]
pub struct SkillBook {
    /// Map from skill name → progress.
    pub skills: HashMap<String, SkillProgress>,
    /// Name of the skill currently in the training queue (if any).
    pub training_skill: Option<String>,
}

impl SkillBook {
    /// Returns the trained level for a skill (0 if never injected).
    pub fn level(&self, skill_name: &str) -> u8 {
        self.skills
            .get(skill_name)
            .map(|p| p.level)
            .unwrap_or(0)
    }

    /// Injects a skill (adds it to the book at level 0 if not present).
    pub fn inject(&mut self, skill_name: &str) {
        self.skills
            .entry(skill_name.to_string())
            .or_insert_with(SkillProgress::default);
    }

    /// Sets the active training skill.
    pub fn start_training(&mut self, skill_name: &str) {
        self.inject(skill_name);
        self.training_skill = Some(skill_name.to_string());
    }

    /// Advances training by `dt` seconds of wall-clock time.
    ///
    /// Returns the name of the skill if it levelled up this tick.
    pub fn tick(&mut self, dt: f32, attributes: &CharacterAttributes) -> Option<String> {
        let training_name = self.training_skill.clone()?;
        let def = SkillDatabase::get(&training_name)?;

        let sp_per_second = attributes.training_speed(def.primary, def.secondary) / 60.0;
        let sp_gained = sp_per_second * dt;

        let progress = self.skills.entry(training_name.clone()).or_default();
        let levelled_up = progress.add_sp(sp_gained, def.rank);

        if levelled_up > 0 {
            if progress.level >= 5 {
                self.training_skill = None;
            }
            Some(training_name)
        } else {
            None
        }
    }
}
