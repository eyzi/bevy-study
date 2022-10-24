# Space Invade.rs

Space Invaders game implementation using guide by
[TheBracket](https://rustrepo.com/repo/thebracket-bevy-test)

## Usage
```
cargo run --example space-invaders
```

## Features
- Four rows of invaders are created at the beginning
- Invaders move sideways and slowly goes down one row at a time
- Player moves left [A] and right [D] and fires laser [Space]
- Invader dies when hit by a laser
- Uses spritesheet

## Flaws
- No game over screen
- No victory screen or score system
- Horizontal movement of invaders is smooth while vertical is sudden
- Single file for all logic

##### space invaders is the hello world of game developers, after all.
