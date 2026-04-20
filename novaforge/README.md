# NovaForge — Fyrox Edition

NovaForge is a PvE-focused sci-fi space MMO inspired by EVE Online, now built on the
[Fyrox](https://fyrox.rs) game engine in Rust.

## Overview

Nova Forge is a cooperative space game for small groups (2–20 players). Core gameplay:

- **Ship system** — Frigates, Destroyers, Cruisers, Battlecruisers, Battleships
- **Slot fitting** — High / Mid / Low slots; turret and launcher hardpoints
- **Four races** — Amarr, Caldari, Gallente, Minmatar with distinct weapon affinities
- **Skill system** — Real-time passive training (continues while offline)
- **PvE combat** — Four damage types (EM / Thermal / Kinetic / Explosive) with resistances
- **Faction standings** — Affects NPC aggression, mission access, and market fees
- **Economy** — Player-driven buy/sell order book with broker fees

## Project Structure

```
novaforge/
  Cargo.toml           # Sub-workspace (references local Fyrox engine)
  data/
    scene.rgs          # Default 3D space scene
  game/                # Game library (Fyrox plugin)
    src/
      lib.rs           # Game plugin entry point
      ship.rs          # Ship classes, stats, slot system
      faction.rs       # Faction definitions and standings
      skills.rs        # Skill database and passive training
      combat.rs        # Damage types, weapons, hit points
      economy.rs       # Market orders, wallet, inventory
      character.rs     # Player character aggregation
      player_ship.rs   # PlayerShip Fyrox script (WASD movement, weapons)
      npc_ship.rs      # NpcShip Fyrox script (approach/orbit AI)
  executor/            # Standalone game runner
    src/main.rs
```

## Building & Running

### Prerequisites

- Rust ≥ 1.87 — install via [rustup.rs](https://rustup.rs)
- Linux: `libasound2-dev`, `libxcb-shape0-dev`, `libxcb-xfixes0-dev`
- macOS / Windows: no additional system libraries required

### Quick start (game only)

```sh
cd novaforge
cargo run --package executor --release
```

### Build everything (engine + game)

From the **repo root**:

```sh
./build.sh             # debug build
./build.sh --release   # release build
./build.sh --game      # game only (skips engine build)
./build.sh --test      # run all tests
```

## Controls

| Key | Action |
|-----|--------|
| W / ↑ | Thrust forward |
| S / ↓ | Thrust backward |
| A / ← | Strafe left |
| D / → | Strafe right |
| Space | Fire primary weapon |
| Esc | Quit |

## Architecture

The game plugin (`novaforge`) is registered with Fyrox's plugin system.
On startup it:

1. Registers `PlayerShip` and `NpcShip` scripts with the serialisation context.
2. Loads `data/scene.rgs` as the initial scene.
3. Starts the passive skill training loop in `Game::update`.

Individual game objects (ships) are scene nodes with a Fyrox `ScriptTrait`
attached. Scripts read input and move nodes each frame. Damage is applied via
the `HitPoints::apply_damage` method inside each script's `on_update`.

## Roadmap

- [ ] Space background (skybox, star field)
- [ ] Procedural asteroid belts and anomalies
- [ ] Station docking UI
- [ ] Full market UI
- [ ] Faction mission system
- [ ] Multiplayer networking
- [ ] Galaxy / sector / system map
