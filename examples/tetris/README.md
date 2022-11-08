# Tetris

Tetris game implementation

## Usage
```
cargo run --example tetris
```

## Controls

```
[Q]   [E]
[A][S][D]
```

- `Q` to rotate anticlockwise
- `E` to rotate clockwise
- `A` to move left
- `S` to move down
- `D` to move right

## Features
- Grid system
- Border and masking system
- Somewhat efficient draw and clear systems
- Gravity system
- Input system
- Tetromino shape and rotation system
- Better project structure
- Usage of window icon
- Block stacking
- Row clearing when filled

## Flaws
- Memory usage seems to grow by about 8Kb per second. I suspect this is
  due to the use of Vectors but I have not confirmed yet (seems like
  this is a memory leak with Camera2dBundle)
- Messy code, as usual
- No game over screen. Game just crashes. I realize near the end that
  the code is so messy, there's no place for cleanly resetting blocks
- The code to clear filled rows can be improved a lot. The grid could
  have been a resource instead of a component
