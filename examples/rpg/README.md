# RPG (bevy 0.9)

Generic RPG implementation

## Usage
```
cargo run --example rpg
```

## Controls

```
   [W]          [I][O]
[A][S][D][F]
```

- `W` to move up
- `A` to move left
- `S` to move down
- `D` to move right
- `F` to toggle fullscreen
- `I` to open/close dialogue box
- `O` to start/end snow

## Features
- States and Menu screens
- Import from crate and use mod.rs for cleaner file structure
- Character movement
- Collisions
- LDTK map import and entity/intgrid parsing
- Snow mechanics
- Dialogue mechanics
- Fullscreen Mode

## Flaws
- Dialogues are not organic interactions
- Map is not loaded in a proper state
- No animation/cut scene mechanics yet
- No map change mechanics yet
- Not really a "game" tbh
