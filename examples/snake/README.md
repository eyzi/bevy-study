# Snake (bevy 0.9)

Snake game implementation using guide by
[mbuffet](https://mbuffett.com/posts/bevy-snake-tutorial/)

## Usage
```
cargo run --example snake
```

## Features
- Snake starts as a head
- One food is spawned at any time and at a random point
- Snake moves directionally [W, A, S, D]
- World wraps around on every direction
- Snake grows each time it eats food

## Flaws
- Food may be spawned on the snake's body
- No game over screen. Hitting own body restarts the game immediately
- Variables are all over the place. Need to be centralized for easy
  tweaking
