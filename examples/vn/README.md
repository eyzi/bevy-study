# VN

VN implementation

## Usage
```
cargo run --example vn
```

## Features
- Text animation
- Sprite change
- BG change
- Speaker and line change
- Dynamic script parsing
- Complete animating line on interaction

## Flaws
- No sprite and BG animation
- No audio work
- I setup a folder to separate the VN plugin but I end up not using it.
  Plan on doing it next time but not for now. All logic are in the
  scene module. Features can be improved further but satisfactory for
  an initial VN plugin.
- The dialogue box doesn't seem to wrap the text well yet. Need to look
  more into it.
