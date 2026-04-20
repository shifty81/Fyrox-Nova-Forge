//! Faction system for NovaForge.
//!
//! Tracks player standing with each faction and NPC pirate group. Standing
//! ranges from -10.0 (hostile) to +10.0 (friendly) and affects mission
//! availability, market access, and NPC aggression.

use fyrox::core::reflect::prelude::*;
use fyrox::core::visitor::prelude::*;
use std::collections::HashMap;

/// All major factions in NovaForge.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Visit, Reflect)]
pub enum Faction {
    // Playable empire factions
    AmarrEmpire,
    CaldariState,
    GallenteRFederation,
    MinmatarRepublic,

    // NPC pirate factions
    Serpentis,
    Guristas,
    BloodRaiders,
    SanshasNation,
    AngelCartel,
    RogueDrones,
}

impl Default for Faction {
    fn default() -> Self {
        Faction::CaldariState
    }
}

impl Faction {
    /// Human-readable display name.
    pub fn display_name(self) -> &'static str {
        match self {
            Faction::AmarrEmpire => "Amarr Empire",
            Faction::CaldariState => "Caldari State",
            Faction::GallenteRFederation => "Gallente Federation",
            Faction::MinmatarRepublic => "Minmatar Republic",
            Faction::Serpentis => "Serpentis",
            Faction::Guristas => "Guristas Pirates",
            Faction::BloodRaiders => "Blood Raiders",
            Faction::SanshasNation => "Sansha's Nation",
            Faction::AngelCartel => "Angel Cartel",
            Faction::RogueDrones => "Rogue Drones",
        }
    }

    /// Returns true if this faction is an NPC pirate group.
    pub fn is_pirate(self) -> bool {
        matches!(
            self,
            Faction::Serpentis
                | Faction::Guristas
                | Faction::BloodRaiders
                | Faction::SanshasNation
                | Faction::AngelCartel
                | Faction::RogueDrones
        )
    }
}

/// Player standings with all factions.
///
/// Standing is clamped to [-10.0, +10.0]. Negative standing causes NPC
/// factions to attack on sight; positive standing improves market prices and
/// unlocks agent missions.
#[derive(Debug, Clone, Visit, Reflect)]
pub struct FactionStandings {
    standings: HashMap<String, f32>,
}

impl Default for FactionStandings {
    fn default() -> Self {
        let mut s = FactionStandings {
            standings: HashMap::new(),
        };
        // Start at neutral (0.0) for empire factions, slightly negative for pirates.
        s.set(Faction::AmarrEmpire, 0.0);
        s.set(Faction::CaldariState, 0.0);
        s.set(Faction::GallenteRFederation, 0.0);
        s.set(Faction::MinmatarRepublic, 0.0);
        s.set(Faction::Serpentis, -2.0);
        s.set(Faction::Guristas, -2.0);
        s.set(Faction::BloodRaiders, -2.0);
        s.set(Faction::SanshasNation, -2.0);
        s.set(Faction::AngelCartel, -2.0);
        s.set(Faction::RogueDrones, -5.0);
        s
    }
}

impl FactionStandings {
    fn key(faction: Faction) -> String {
        format!("{faction:?}")
    }

    /// Returns the current standing with a faction (-10.0 to +10.0).
    pub fn get(&self, faction: Faction) -> f32 {
        *self.standings.get(&Self::key(faction)).unwrap_or(&0.0)
    }

    /// Sets the standing, clamping to [-10.0, +10.0].
    pub fn set(&mut self, faction: Faction, value: f32) {
        self.standings
            .insert(Self::key(faction), value.clamp(-10.0, 10.0));
    }

    /// Modifies standing by a delta, respecting the clamp.
    pub fn modify(&mut self, faction: Faction, delta: f32) {
        let current = self.get(faction);
        self.set(faction, current + delta);
    }

    /// Returns true if the player is KOS (kill-on-sight) for the faction.
    pub fn is_kos(&self, faction: Faction) -> bool {
        self.get(faction) <= -5.0
    }

    /// Returns the broker fee multiplier based on standing (0.01 to 0.05).
    pub fn broker_fee(&self, faction: Faction) -> f32 {
        let standing = self.get(faction);
        let base = 0.05_f32;
        (base - standing * 0.003).max(0.01)
    }
}
