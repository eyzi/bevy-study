use super::super::core::config;
use super::block::{create as create_block, *};
use super::coords::*;
use bevy::prelude::*;
use bevy::utils::HashMap;

pub const BLANK_COLOR: Color = Color::rgb(0.06, 0.06, 0.06);
pub const BORDER_COLOR: Color = Color::rgb(0.4, 0.4, 0.4);

#[derive(Component)]
pub struct Grid {
    pub blocks: HashMap<Coords, Entity>,
}

pub fn setup(mut commands: Commands) {
    let mut block_dictionary = Grid {
        blocks: HashMap::new(),
    };

    let columns = config::WINDOW_WIDTH / config::CELL_SIZE;
    let rows = config::WINDOW_HEIGHT / config::CELL_SIZE;
    for x in 0..columns as i8 {
        for y in 0..rows as i8 {
            let coords = Coords { x, y };
            let is_border = is_border(coords);
            let color = if is_border { BORDER_COLOR } else { BLANK_COLOR };

            let block = create_block(&mut commands, coords, color);

            block_dictionary.blocks.insert(coords, block);

            if is_border && y != border_top() {
                set_collidable(&mut commands, block);
            }
        }
    }

    commands.spawn().insert(block_dictionary);
}

pub fn is_border(coords: Coords) -> bool {
    border_vertical(coords) || border_horizontal(coords)
}

pub fn screen_top() -> i8 {
    (config::WINDOW_HEIGHT / config::CELL_SIZE) as i8
}

pub fn screen_right() -> i8 {
    (config::WINDOW_WIDTH / config::CELL_SIZE) as i8
}

pub fn border_bottom() -> i8 {
    ((config::WINDOW_HEIGHT / config::CELL_SIZE - config::GRID_HEIGHT as f32) / 2. - 1.) as i8
}

pub fn border_left() -> i8 {
    ((config::WINDOW_WIDTH / config::CELL_SIZE - config::GRID_WIDTH as f32) / 2. - 1.) as i8
}

pub fn border_top() -> i8 {
    border_bottom() + 21
}

pub fn border_right() -> i8 {
    border_left() + 11
}

pub fn border_vertical(coords: Coords) -> bool {
    coords.y >= border_bottom()
        && coords.y <= border_top()
        && (coords.x == border_left() || coords.x == border_right())
}

pub fn border_horizontal(coords: Coords) -> bool {
    coords.x >= border_left()
        && coords.x <= border_right()
        && (coords.y == border_top() || coords.y == border_bottom())
}

pub fn within_screen(x: i16, y: i16) -> bool {
    x >= 0i16 && x <= screen_right() as i16 && y >= 0i16 && y <= screen_top() as i16
}

pub fn within_border(x: i16, y: i16) -> bool {
    x > border_left() as i16
        && x < border_right() as i16
        && y > border_bottom() as i16
        && y < border_top() as i16
}
