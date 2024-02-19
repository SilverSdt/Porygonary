# Porygonary - Pokemon Simulator

- [Porygonary - Pokemon Simulator](#porygonary---pokemon-simulator)
  - [Planned Features for v1.0.0](#planned-features-for-v100)
  - [Overview](#overview)
    - [Understanding Types](#understanding-types)
    - [Understanding Moves](#understanding-moves)
    - [Understanding Abilities](#understanding-abilities)
    - [Understanding Pokemon](#understanding-pokemon)
  - [Contributors](#contributors)
  - [License](#license)

## Planned Features for v1.0.0

- Accurate damage calculation
- Full support for types, including dual types, resistances, weaknesses, and immunities
- Full support for moves
- Abilities
- Pre-made Pokemon

## Overview

Porygonary is a Rust-based project designed to simulate Pokemon battles with accuracy, mirroring the mechanics of the actual Pokemon games.

### Understanding Types

In Pokemon, a 'type' represents an element (such as fire, grass, water) that each pokemon can have. Each type has its own strengths and weaknesses against other types.

For instance:

- **Fire** types are weak to **water** but strong against **grass**
- **Water** types are weak to **grass** but strong against **fire**
- **Grass** types are weak to **fire** but strong against **water**

If a type is weak to another, damage inflicted is doubled. If it's resistant, damage is halved. If it's immune, no damage is taken.

### Understanding Moves

A 'move' is a special ability that a pokemon can use during battle to **inflict damage**, **boost their stats**, or to **inflict status effects**.

Each move has a **base power**, an **accuracy**, and an optional **effect**. A move can be a **special move**, **physical move**, or a **status move**. It is precise if **contact** is established between the user and the target when using the ability.

### Understanding Abilities

An 'ability' is a passive effect that a pokemon can use in a battle. Abilities have positive and negative effects, and can be triggered in different ways.

For example:

- **Blaze:** Increases the power of fire-type moves by 50% when the Pokemon's HP falls below a third of its maximum.
- **Rough Skin:** Damages the opponent when they make contact, inflicting damage equal to 1/8 of their maximum HP.
- **Levitate:** Makes the Pokemon immune to ground-type moves, spikes, toxic spikes and arena trap.

### Understanding Pokemon

(_Who doesn't know what a Pokemon is?_)

A Pokemon is a creature that can be used in battle. Each Pokemon has:

- A **name**
- 1 or 2 **types**
- A **level** (ranging from 1 to 100)
- Base **statistics**
- **IVs** (Individual Values, ranging from 0 to 31 per stat)
- **EVs** (Effort Values, up to 252 per stat & 510 in total)
- An **ability**
- 1 to 4 **moves**

## Contributors

- [Jérémy Bodart](https://jeremy.bodart.dev)
- [Joshua Vandaële](https://vandaele.software)

## License

This project is under the GNU General Public License v3.0 - see the [LICENSE](LICENSE) file for details.
