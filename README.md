# **Porygonary - PokemonSimulator**

## Author
[Jérémy Bodart](https://jeremy.bodart.dev)

## Description
Porygonary - Pokemon Simulator is a rust project with goal to give tool to simulate a pokemon battle like a real pokemon game.

## V1.0.0 Features
- Type
- Move
- Abilite
- Pokemon
- Damage Calculation

### What is a type ?
A type represented a element (fire, grass, water...) that have weakness, resistance or immunity to a other type.

**for examples:**
- **Fire** is weak against **water** but resistant to **grass**
- **Water** is weak against **grass** but resistant to **fire**
- **Grass** is weak against **fire** but resistant to **water**

if the type is weak, the damage is doubled, if it is resistant, it is divised by 2, and if it is immune, the damage is null.

### What is a move ?
A move is a special abilitie that can use a pokemon during battle for **inflict damage**, **boost their stats** or **status.**

A move have a **base power**, a **accuracy** and a optional **effect**. A move can be a **special move, physique move or a status move** and it is precise if **contact** is established between him and the other pokemon when using the ability.

### What is a abilitie ?
A abilities is a passive effect that a pokemon use during battle, their can be positive or negative.

**for exemples:**
- **Blaze:** Powers up fire-types moves by 50% when the HP falls below a third of its maximum.
- **Rough Skin:** inflict damage as the opponent pokemon when opponent move make contact and inflict 1/8 of their maximum HP.
- **Levitate:** immune the pokemon from ground-type moves, spikes, toxic spikes or arena trap.

### What is a pokemon ?
(_Who doesn't know what is a pokemon?_)

A pokemon is a creature you can use in battle,
you can define a pokemon with:
- A **name**
- 1 or 2 **types**
- A **level** (1 to 100)
- Base **statistics**
- **IV** (0 to 31 per stats)
- **EV** (252 max per stats & 510 total max)
- **Abilitie**
- 1 to 4 **moves**


