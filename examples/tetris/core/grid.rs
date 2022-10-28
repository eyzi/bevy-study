use std::sync::Arc;

use super::cell::Cell;
use super::config;
use super::tetris::*;
use super::tetromino::*;
use bevy::prelude::*;
use bevy::sprite::Anchor;

#[derive(Component, Clone)]
pub struct Grid(Vec<Vec<Cell>>);

const BLANK_COLOR: Color = Color::rgb(0.06, 0.06, 0.06);
const HALF_WINDOW_WIDTH: f32 = config::WINDOW_WIDTH / 2.;
const HALF_WINDOW_HEIGHT: f32 = config::WINDOW_HEIGHT / 2.;
const CELL_SIZE: f32 = 20.;
const CELL_MARGIN: f32 = 1.;

impl Grid {
    pub fn new() -> Self {
        return Grid(vec![
            vec![
                Cell::new(BLANK_COLOR);
                (config::WINDOW_WIDTH / CELL_SIZE) as usize
            ];
            (config::WINDOW_HEIGHT / CELL_SIZE) as usize
        ]);
    }

    pub fn set_cell(&mut self, x: usize, y: usize, cell: Cell) {
        let cells = &mut self.0;
        cells[x][y] = cell;
    }
}

pub fn setup(mut commands: Commands) {
    let mut grid = Grid::new();

    draw_border(&mut grid);

    commands.spawn().insert(grid);
    commands.spawn().insert(Thing { x: 5, y: 20 });
}

pub fn draw_border(grid: &mut Grid) {
    let border_color = Color::rgb(0.5, 0.5, 0.5);
    let border_cell = Cell::new(border_color);

    for x in 0..=21 {
        grid.set_cell(x, 5, border_cell);
        grid.set_cell(x, 16, border_cell);
    }

    for y in 6..=15 {
        grid.set_cell(0, y, border_cell);
        grid.set_cell(21, y, border_cell);
    }
}

pub fn draw(
    mut commands: Commands,
    mut grid_query: Query<&mut Grid>,
    mut tetris_query: Query<&mut Tetris>,
) {
    let mut grid = grid_query.iter_mut().next().unwrap();
    let mut tetris = tetris_query.iter_mut().next().unwrap();

    for (y, rows) in grid.0.iter_mut().enumerate() {
        for (x, cell) in rows.iter().enumerate() {
            commands
                .spawn()
                .insert(cell.clone())
                .insert_bundle(SpriteBundle {
                    sprite: Sprite {
                        color: cell.color,
                        anchor: Anchor::Center,
                        ..default()
                    },
                    transform: Transform {
                        scale: Vec3::new(CELL_SIZE - CELL_MARGIN, CELL_SIZE - CELL_MARGIN, 1.),
                        translation: Vec3::new(
                            (CELL_SIZE * x as f32) - (HALF_WINDOW_WIDTH - (CELL_SIZE / 2.)),
                            (CELL_SIZE * y as f32) - (HALF_WINDOW_HEIGHT - (CELL_SIZE / 2.)),
                            0.,
                        ),
                        ..default()
                    },
                    ..default()
                });
        }
    }

    let shape: TetrominoInPlay = TetrominoInPlay {
        tetromino: Tetromino::new(TetrominoShape::I),
        x: 4,
        y: 4,
    };

    for mut gridcell in shape.get_cells().iter_mut() {
        grid.0[gridcell.x as usize][gridcell.y as usize] = gridcell.cell;
    }
}

#[derive(Component, Clone, Copy)]
pub struct Thing {
    x: u8,
    y: u8,
}

pub fn apply_gravity(
    mut commands: Commands,
    mut query: Query<&mut Grid>,
    mut thing_query: Query<(Entity, &Thing)>,
) {
    let mut grid = query.iter_mut().next().unwrap();
    if let Some((old_thing_entity, &old_thing)) = thing_query.iter_mut().next() {
        let new_y = if old_thing.y > 0 { old_thing.y - 1 } else { 20 };
        let color = if new_y < 20 {
            Color::YELLOW
        } else {
            Color::rgba(0., 0., 0., 0.)
        };
        commands
            .spawn()
            .insert(Thing {
                x: old_thing.x,
                y: new_y,
            })
            .insert_bundle(SpriteBundle {
                sprite: Sprite {
                    color,
                    anchor: Anchor::BottomLeft,
                    ..default()
                },
                transform: Transform {
                    scale: Vec3::new(19., 19., 1.),
                    translation: Vec3::new(
                        (20. * old_thing.x as f32) - (100.),
                        (20. * new_y as f32) - (200.),
                        0.,
                    ),
                    ..default()
                },
                ..default()
            });

        commands.entity(old_thing_entity).despawn();
    }
}
