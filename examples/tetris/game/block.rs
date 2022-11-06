use super::super::core::config;
use super::collission::*;
use super::coords::*;
use super::gravity::*;
use super::screen::*;
use super::tetromino;
use super::tetromino::*;
use bevy::prelude::*;
use bevy::sprite::Anchor;

#[derive(Component, Clone, Copy)]
pub struct Block {
    pub coords: Coords,
    pub color: Color,
}

#[derive(Component)]
pub struct Clearable;

impl Block {
    fn screen_coords(&self) -> (f32, f32) {
        let screen_x = -config::HALF_WINDOW_WIDTH + self.coords.x as f32 * config::CELL_SIZE;
        let screen_y = -config::HALF_WINDOW_HEIGHT + self.coords.y as f32 * config::CELL_SIZE;
        (screen_x, screen_y)
    }

    pub fn sprite_bundle(&self) -> SpriteBundle {
        let (screen_x, screen_y) = self.screen_coords();

        SpriteBundle {
            sprite: Sprite {
                color: self.color,
                anchor: Anchor::BottomLeft,
                ..default()
            },
            transform: Transform {
                scale: Vec3::new(
                    config::CELL_SIZE - config::CELL_MARGIN,
                    config::CELL_SIZE - config::CELL_MARGIN,
                    1.,
                ),
                translation: Vec3::new(screen_x, screen_y, 0.),
                ..default()
            },
            ..default()
        }
    }
}

pub fn create(commands: &mut Commands, coords: Coords, color: Color) -> Entity {
    let block = Block { coords, color };
    commands
        .spawn()
        .insert(block.clone())
        .insert_bundle(block.sprite_bundle())
        .id()
}

pub fn set_collidable(commands: &mut Commands, block: Entity) {
    commands.entity(block).insert(Collidable);
}

pub fn persist_tetromino(
    commands: &mut Commands,
    grid: &Grid,
    tetromino: Tetromino,
    origin_x: i8,
    origin_y: i8,
) {
    for (x, row) in tetromino.cells().into_iter().enumerate() {
        for (y, cell) in row.into_iter().enumerate() {
            if let Some(color) = cell {
                if y as i8 >= border_top() {
                    // game over
                }

                let block = Block {
                    coords: Coords {
                        x: origin_x + x as i8,
                        y: origin_y + y as i8,
                    },
                    color,
                };
                let block_entity = grid.blocks.get(&block.coords).unwrap();
                commands
                    .entity(*block_entity)
                    .insert_bundle(block.sprite_bundle())
                    .insert(Collidable)
                    .insert(Clearable);
            }
        }
    }
}

pub fn refresh(
    mut commands: Commands,
    block_query: Query<&Block>,
    grid_query: Query<&Grid>,
    held_query: Query<&Tetromino, With<Held>>,
    upcoming_query: Query<(&Tetromino, &Upcoming, Without<Falling>, Without<Held>)>,
    falling_query: Query<(&Tetromino, &Falling, Without<Upcoming>, Without<Held>)>,
) {
    let grid = grid_query.single();

    for tetromino in held_query.iter() {
        let origin_x = border_left() - 5;
        let origin_y = border_top() - 3;

        for (x, row) in tetromino.cells().into_iter().enumerate() {
            for (y, cell) in row.into_iter().enumerate() {
                let mut color = BLANK_COLOR;
                if let Some(c) = cell {
                    color = c;
                }

                let block_x = origin_x + x as i8;
                let block_y = origin_y + y as i8;

                if within_screen(block_x as i16, block_y as i16) {
                    let block = Block {
                        coords: Coords {
                            x: block_x,
                            y: block_y,
                        },
                        color,
                    };
                    let block_entity = grid.blocks.get(&block.coords).unwrap();
                    commands
                        .entity(*block_entity)
                        .insert_bundle(block.sprite_bundle());
                }
            }
        }
    }

    for (tetromino, upcoming, _, _) in upcoming_query.iter() {
        let origin_x = border_right() + 2;
        let origin_y = border_top() - 3 - (4 * upcoming.index);

        for (x, row) in tetromino.cells().into_iter().enumerate() {
            for (y, cell) in row.into_iter().enumerate() {
                let mut color = BLANK_COLOR;
                if let Some(c) = cell {
                    color = c;
                }

                let block_x = origin_x + x as i8;
                let block_y = origin_y + y as i8;

                if within_screen(block_x as i16, block_y as i16) {
                    let block = Block {
                        coords: Coords {
                            x: block_x,
                            y: block_y,
                        },
                        color,
                    };
                    let block_entity = grid.blocks.get(&block.coords).unwrap();
                    commands
                        .entity(*block_entity)
                        .insert_bundle(block.sprite_bundle());
                }
            }
        }
    }

    for (tetromino, falling, _, _) in falling_query.iter() {
        let origin_x = falling.coords.x;
        let origin_y = falling.coords.y;

        for (x, row) in tetromino.cells().into_iter().enumerate() {
            for (y, cell) in row.into_iter().enumerate() {
                if let Some(color) = cell {
                    let block_x = origin_x + x as i8;
                    let block_y = origin_y + y as i8;

                    if within_border(block_x as i16, block_y as i16) {
                        let block = Block {
                            coords: Coords {
                                x: block_x,
                                y: block_y,
                            },
                            color,
                        };
                        let block_entity = grid.blocks.get(&block.coords).unwrap();
                        commands
                            .entity(*block_entity)
                            .insert_bundle(block.sprite_bundle());
                    }
                }
            }
        }
    }
}
