//! Ship definitions, statistics, and slot system for NovaForge.
//!
//! Ships are the primary vehicle for all gameplay. Each ship has:
//! - Hull, Shield, and Armor hit points
//! - High / Mid / Low slot counts for module fitting
//! - Turret and Launcher hardpoints
//! - Racial bonuses and role bonuses

use fyrox::core::reflect::prelude::*;
use fyrox::core::visitor::prelude::*;

/// Playable and NPC ship classes, ordered by size.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Visit, Reflect)]
pub enum ShipClass {
    Frigate,
    Destroyer,
    Cruiser,
    Battlecruiser,
    Battleship,
    MiningBarge,
    Capital,
}

impl Default for ShipClass {
    fn default() -> Self {
        ShipClass::Frigate
    }
}

/// The four playable races, each with distinct weapon and tanking affinities.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Visit, Reflect)]
pub enum Race {
    /// Energy turrets, armor tanking.
    Amarr,
    /// Hybrid turrets & missiles, shield tanking.
    Caldari,
    /// Hybrid turrets & drones, armor tanking.
    Gallente,
    /// Projectile turrets, speed-based defense.
    Minmatar,
}

impl Default for Race {
    fn default() -> Self {
        Race::Caldari
    }
}

/// Resistance profile for a ship layer (shield, armor, hull).
/// Values are in 0.0–1.0 where 1.0 = full resistance.
#[derive(Debug, Clone, Default, Visit, Reflect)]
pub struct ResistanceProfile {
    pub em: f32,
    pub thermal: f32,
    pub kinetic: f32,
    pub explosive: f32,
}

/// Base statistics for a ship type before module fitting is applied.
#[derive(Debug, Clone, Visit, Reflect)]
pub struct ShipStats {
    pub class: ShipClass,
    pub race: Race,

    // Hit points
    pub hull_hp: f32,
    pub shield_hp: f32,
    pub armor_hp: f32,

    // Resistances
    pub shield_resists: ResistanceProfile,
    pub armor_resists: ResistanceProfile,
    pub hull_resists: ResistanceProfile,

    // Fitting resources
    pub cpu: f32,
    pub powergrid: f32,
    pub capacitor: f32,

    // Fitting slots
    pub high_slots: u8,
    pub mid_slots: u8,
    pub low_slots: u8,

    // Hardpoints
    pub turret_hardpoints: u8,
    pub launcher_hardpoints: u8,

    // Navigation
    pub max_velocity: f32,
    pub agility: f32,
    pub signature_radius: f32,

    // Drone bay
    pub drone_bandwidth: u8,
    pub drone_bay_capacity: u8,
}

impl Default for ShipStats {
    fn default() -> Self {
        ShipStats {
            class: ShipClass::Frigate,
            race: Race::Caldari,
            hull_hp: 450.0,
            shield_hp: 600.0,
            armor_hp: 450.0,
            shield_resists: ResistanceProfile { em: 0.0, thermal: 0.20, kinetic: 0.25, explosive: 0.50 },
            armor_resists: ResistanceProfile { em: 0.50, thermal: 0.35, kinetic: 0.25, explosive: 0.10 },
            hull_resists: ResistanceProfile::default(),
            cpu: 170.0,
            powergrid: 45.0,
            capacitor: 550.0,
            high_slots: 3,
            mid_slots: 4,
            low_slots: 2,
            turret_hardpoints: 2,
            launcher_hardpoints: 1,
            max_velocity: 380.0,
            agility: 3.5,
            signature_radius: 34.0,
            drone_bandwidth: 0,
            drone_bay_capacity: 0,
        }
    }
}

/// Catalogue of all available ship types in NovaForge.
pub struct ShipDatabase;

impl ShipDatabase {
    /// Returns the base stats for a given ship type name.
    /// Returns a default Merlin-class frigate when the type is unknown.
    pub fn stats_for(ship_type: &str) -> ShipStats {
        match ship_type {
            // Caldari frigates
            "Merlin" => ShipStats {
                class: ShipClass::Frigate,
                race: Race::Caldari,
                hull_hp: 450.0,
                shield_hp: 750.0,
                armor_hp: 450.0,
                high_slots: 3,
                mid_slots: 4,
                low_slots: 2,
                turret_hardpoints: 2,
                launcher_hardpoints: 1,
                cpu: 170.0,
                powergrid: 45.0,
                capacitor: 550.0,
                max_velocity: 380.0,
                agility: 3.5,
                signature_radius: 34.0,
                ..ShipStats::default()
            },
            // Caldari destroyers
            "Cormorant" => ShipStats {
                class: ShipClass::Destroyer,
                race: Race::Caldari,
                hull_hp: 600.0,
                shield_hp: 1000.0,
                armor_hp: 550.0,
                high_slots: 7,
                mid_slots: 3,
                low_slots: 2,
                turret_hardpoints: 7,
                launcher_hardpoints: 0,
                cpu: 220.0,
                powergrid: 62.0,
                capacitor: 750.0,
                max_velocity: 320.0,
                agility: 4.2,
                signature_radius: 55.0,
                ..ShipStats::default()
            },
            // Caldari cruisers
            "Caracal" => ShipStats {
                class: ShipClass::Cruiser,
                race: Race::Caldari,
                hull_hp: 1300.0,
                shield_hp: 2100.0,
                armor_hp: 1200.0,
                high_slots: 5,
                mid_slots: 5,
                low_slots: 3,
                turret_hardpoints: 0,
                launcher_hardpoints: 4,
                cpu: 365.0,
                powergrid: 700.0,
                capacitor: 2250.0,
                max_velocity: 240.0,
                agility: 5.2,
                signature_radius: 135.0,
                ..ShipStats::default()
            },
            // Amarr frigates
            "Punisher" => ShipStats {
                class: ShipClass::Frigate,
                race: Race::Amarr,
                hull_hp: 550.0,
                shield_hp: 400.0,
                armor_hp: 800.0,
                high_slots: 4,
                mid_slots: 2,
                low_slots: 4,
                turret_hardpoints: 4,
                launcher_hardpoints: 0,
                cpu: 145.0,
                powergrid: 56.0,
                capacitor: 800.0,
                max_velocity: 330.0,
                agility: 3.3,
                signature_radius: 36.0,
                armor_resists: ResistanceProfile { em: 0.60, thermal: 0.35, kinetic: 0.25, explosive: 0.10 },
                ..ShipStats::default()
            },
            // Gallente frigates
            "Incursus" => ShipStats {
                class: ShipClass::Frigate,
                race: Race::Gallente,
                hull_hp: 500.0,
                shield_hp: 450.0,
                armor_hp: 700.0,
                high_slots: 3,
                mid_slots: 2,
                low_slots: 3,
                turret_hardpoints: 3,
                launcher_hardpoints: 0,
                cpu: 135.0,
                powergrid: 57.0,
                capacitor: 600.0,
                max_velocity: 400.0,
                agility: 3.8,
                signature_radius: 32.0,
                drone_bandwidth: 10,
                drone_bay_capacity: 10,
                ..ShipStats::default()
            },
            // Minmatar frigates
            "Rifter" => ShipStats {
                class: ShipClass::Frigate,
                race: Race::Minmatar,
                hull_hp: 450.0,
                shield_hp: 550.0,
                armor_hp: 500.0,
                high_slots: 4,
                mid_slots: 3,
                low_slots: 2,
                turret_hardpoints: 3,
                launcher_hardpoints: 1,
                cpu: 150.0,
                powergrid: 42.0,
                capacitor: 500.0,
                max_velocity: 440.0,
                agility: 3.2,
                signature_radius: 32.0,
                ..ShipStats::default()
            },
            _ => ShipStats::default(),
        }
    }
}
