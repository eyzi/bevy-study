# Tetris (unfinished)

Tetris game implementation

## Usage
```
cargo run --example tetris
```

## Features
- Grid system
- Border and masking system
- Somewhat efficient draw and clear systems
- Gravity system
- Input system
- Tetromino shape and rotation system
- Better project structure
- Usage of window icon

## Flaws
- Memory usage seems to grow by about 8Kb per second. I suspect this is
  due to the use of Vectors but I have not confirmed yet
- No collission system yet
- No popping of next tetromino yet
