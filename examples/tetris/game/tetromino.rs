use super::block::*;
use super::coords::*;
use super::screen::*;
use bevy::prelude::*;
use rand::{prelude::thread_rng, Rng};

#[derive(Component)]
pub struct Held;

#[derive(Component)]
pub struct Upcoming {
    pub index: u8,
}

#[allow(dead_code)]
#[derive(Clone, Copy)]
pub enum TetrominoShape {
    I,
    O,
    L,
    J,
    S,
    Z,
    T,
    All,
}

impl TetrominoShape {
    pub fn cells(&self) -> Vec<Vec<Option<Color>>> {
        match self {
            TetrominoShape::I => vec![
                vec![None, None, None, None],
                vec![
                    Some(Color::YELLOW),
                    Some(Color::YELLOW),
                    Some(Color::YELLOW),
                    Some(Color::YELLOW),
                ],
                vec![None, None, None, None],
                vec![None, None, None, None],
            ],
            TetrominoShape::T => vec![
                vec![None, None, None, None],
                vec![
                    Some(Color::PURPLE),
                    Some(Color::PURPLE),
                    Some(Color::PURPLE),
                    None,
                ],
                vec![None, Some(Color::PURPLE), None, None],
                vec![None, None, None, None],
            ],
            TetrominoShape::L => vec![
                vec![None, None, None, None],
                vec![None, None, Some(Color::BLUE), None],
                vec![
                    Some(Color::BLUE),
                    Some(Color::BLUE),
                    Some(Color::BLUE),
                    None,
                ],
                vec![None, None, None, None],
            ],
            TetrominoShape::J => vec![
                vec![None, None, None, None],
                vec![
                    Some(Color::GREEN),
                    Some(Color::GREEN),
                    Some(Color::GREEN),
                    None,
                ],
                vec![None, None, Some(Color::GREEN), None],
                vec![None, None, None, None],
            ],
            TetrominoShape::O => vec![
                vec![None, None, None, None],
                vec![None, Some(Color::SEA_GREEN), Some(Color::SEA_GREEN), None],
                vec![None, Some(Color::SEA_GREEN), Some(Color::SEA_GREEN), None],
                vec![None, None, None, None],
            ],
            TetrominoShape::Z => vec![
                vec![None, None, None, None],
                vec![Some(Color::ORANGE), Some(Color::ORANGE), None, None],
                vec![None, Some(Color::ORANGE), Some(Color::ORANGE), None],
                vec![None, None, None, None],
            ],
            TetrominoShape::S => vec![
                vec![None, None, None, None],
                vec![None, Some(Color::PINK), Some(Color::PINK), None],
                vec![Some(Color::PINK), Some(Color::PINK), None, None],
                vec![None, None, None, None],
            ],
            _ => vec![
                vec![
                    Some(Color::RED),
                    Some(Color::RED),
                    Some(Color::RED),
                    Some(Color::RED),
                ],
                vec![
                    Some(Color::RED),
                    Some(Color::RED),
                    Some(Color::RED),
                    Some(Color::RED),
                ],
                vec![
                    Some(Color::RED),
                    Some(Color::RED),
                    Some(Color::RED),
                    Some(Color::RED),
                ],
                vec![
                    Some(Color::RED),
                    Some(Color::RED),
                    Some(Color::RED),
                    Some(Color::RED),
                ],
            ],
        }
    }
}

#[allow(dead_code)]
#[derive(Clone, Copy)]
pub enum TetrominoRotation {
    Zero,
    Ninety,
    OneEighty,
    TwoSeventy,
}

impl TetrominoRotation {
    pub fn clockwise(self) -> Self {
        match self {
            TetrominoRotation::Zero => TetrominoRotation::Ninety,
            TetrominoRotation::Ninety => TetrominoRotation::OneEighty,
            TetrominoRotation::OneEighty => TetrominoRotation::TwoSeventy,
            TetrominoRotation::TwoSeventy => TetrominoRotation::Zero,
        }
    }

    pub fn anticlockwise(self) -> Self {
        match self {
            TetrominoRotation::Zero => TetrominoRotation::TwoSeventy,
            TetrominoRotation::Ninety => TetrominoRotation::Zero,
            TetrominoRotation::OneEighty => TetrominoRotation::Ninety,
            TetrominoRotation::TwoSeventy => TetrominoRotation::OneEighty,
        }
    }
}

#[derive(Component, Clone)]
pub struct Tetromino {
    pub shape: TetrominoShape,
    pub rotation: TetrominoRotation,
}

impl Tetromino {
    pub fn new(shape: TetrominoShape) -> Self {
        Tetromino {
            shape,
            rotation: TetrominoRotation::Zero,
        }
    }

    pub fn cells(&self) -> Vec<Vec<Option<Color>>> {
        self.apply_rotation()
    }

    pub fn rotate_clockwise(self: &mut Self) {
        self.rotation = self.rotation.clockwise();
    }

    pub fn rotate_anticlockwise(self: &mut Self) {
        self.rotation = self.rotation.anticlockwise();
    }

    fn apply_rotation(&self) -> Vec<Vec<Option<Color>>> {
        let cells = self.shape.cells();
        match self.rotation {
            TetrominoRotation::Zero => {
                let width = cells.len();
                let height = cells[0].len();

                let mut new_cells: Vec<Vec<Option<Color>>> = vec![];
                for x in 0..width {
                    let mut new_row: Vec<Option<Color>> = vec![];
                    for y in (0..height).rev() {
                        new_row.push(cells[y][x]);
                    }
                    new_cells.push(new_row);
                }

                new_cells
            }
            TetrominoRotation::Ninety => {
                let width = cells.len();
                let height = cells[0].len();

                let mut new_cells: Vec<Vec<Option<Color>>> = vec![];
                for x in (0..width).rev() {
                    let mut new_row: Vec<Option<Color>> = vec![];
                    for y in (0..height).rev() {
                        new_row.push(cells[x][y]);
                    }
                    new_cells.push(new_row);
                }

                new_cells
            }
            TetrominoRotation::OneEighty => {
                let width = cells.len();
                let height = cells[0].len();

                let mut new_cells: Vec<Vec<Option<Color>>> = vec![];
                for x in (0..height).rev() {
                    let mut new_row: Vec<Option<Color>> = vec![];
                    for y in 0..width {
                        new_row.push(cells[y][x]);
                    }
                    new_cells.push(new_row);
                }

                new_cells
            }
            _ => cells,
        }
    }
}

pub fn setup(mut commands: Commands) {
    for index in 0..3 {
        create_upcoming(&mut commands, index, Tetromino::new(random_shape()))
    }
}

pub fn create_held(commands: &mut Commands, tetromino: Tetromino) {
    commands.spawn().insert(Held).insert(tetromino);
}

pub fn create_upcoming(commands: &mut Commands, index: u8, tetromino: Tetromino) {
    commands
        .spawn()
        .insert(Upcoming { index })
        .insert(tetromino);
}

pub fn random_shape() -> TetrominoShape {
    let mut rng = thread_rng();
    match rng.gen_range(0..=6i8) {
        0 => TetrominoShape::J,
        1 => TetrominoShape::L,
        2 => TetrominoShape::O,
        3 => TetrominoShape::Z,
        4 => TetrominoShape::S,
        5 => TetrominoShape::T,
        _ => TetrominoShape::I,
    }
}

pub fn clear(
    commands: &mut Commands,
    grid: &Grid,
    tetromino: &Tetromino,
    origin_x: u8,
    origin_y: u8,
) {
    for (x, row) in tetromino.cells().into_iter().enumerate() {
        for (y, _cell) in row.into_iter().enumerate() {
            let block_x = origin_x + x as u8;
            let block_y = origin_y + y as u8;

            if within_border(block_x as i16, block_y as i16) {
                let block = Block {
                    coords: Coords {
                        x: block_x,
                        y: block_y,
                    },
                    color: BLANK_COLOR,
                };

                if let Some(block_entity) = grid.blocks.get(&block.coords) {
                    commands
                        .entity(*block_entity)
                        .insert_bundle(block.sprite_bundle());
                }
            }
        }
    }
}
