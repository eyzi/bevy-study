use super::config;
use bevy::prelude::*;
use bevy::sprite::Anchor;

#[derive(Component, Clone)]
pub struct CellInGrid {
    pub cell: Cell,
    pub x: i8,
    pub y: i8,
}

#[derive(Component, Debug, Clone, Copy)]
pub struct Cell {
    pub color: Color,
}

impl Cell {
    pub fn new(color: Color) -> Self {
        Cell { color }
    }

    pub fn draw(self, x: usize, y: usize) -> SpriteBundle {
        SpriteBundle {
            sprite: Sprite {
                color: self.color,
                anchor: Anchor::Center,
                ..default()
            },
            transform: Transform {
                scale: Vec3::new(
                    config::CELL_SIZE - config::CELL_MARGIN,
                    config::CELL_SIZE - config::CELL_MARGIN,
                    1.,
                ),
                translation: Vec3::new(
                    (config::CELL_SIZE * x as f32)
                        - (config::HALF_WINDOW_WIDTH - (config::CELL_SIZE / 2.)),
                    (config::CELL_SIZE * y as f32)
                        - (config::HALF_WINDOW_HEIGHT - (config::CELL_SIZE / 2.)),
                    0.,
                ),
                ..default()
            },
            ..default()
        }
    }
}
