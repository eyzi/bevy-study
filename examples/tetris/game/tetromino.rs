use super::block::*;
use super::coords::*;
use super::gravity::*;
use super::screen::*;
use bevy::prelude::*;
use rand::{prelude::thread_rng, Rng};

#[derive(Component)]
pub struct Held;

#[derive(Component)]
pub struct Upcoming {
    pub index: i8,
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

    pub fn set_rotation(self: &mut Self, rotation: TetrominoRotation) {
        self.rotation = rotation;
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
    commands.spawn(Held).insert(tetromino);
}

pub fn create_upcoming(commands: &mut Commands, index: i8, tetromino: Tetromino) {
    commands.spawn(Upcoming { index }).insert(tetromino);
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

pub fn populate_falling(
    mut commands: Commands,
    falling_query: Query<(Entity, &mut Tetromino, &mut Falling), Without<Upcoming>>,
    mut upcoming_query: Query<(
        Entity,
        &mut Tetromino,
        &mut Upcoming,
        Without<Falling>,
        Without<Held>,
    )>,
) {
    if falling_query.is_empty() {
        pop_upcoming(&mut commands, &mut upcoming_query);
    }
}

pub fn pop_upcoming(
    mut commands: &mut Commands,
    upcoming_query: &mut Query<(
        Entity,
        &mut Tetromino,
        &mut Upcoming,
        Without<Falling>,
        Without<Held>,
    )>,
) {
    for (upcoming_entity, tetromino, mut upcoming, _, _) in upcoming_query.iter_mut() {
        if upcoming.index == 0 {
            create_falling(&mut commands, tetromino.clone());
            commands.entity(upcoming_entity).despawn();
        } else {
            upcoming.index -= 1;
        }
    }
    create_upcoming(&mut commands, 2, Tetromino::new(random_shape()));
}

pub fn clear(
    commands: &mut Commands,
    grid: &Grid,
    tetromino: &Tetromino,
    origin_x: i8,
    origin_y: i8,
) {
    for (x, row) in tetromino.cells().into_iter().enumerate() {
        for (y, cell) in row.into_iter().enumerate() {
            if let Some(_color) = cell {
                let block_x = origin_x + x as i8;
                let block_y = origin_y + y as i8;

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
                            .remove_bundle::<SpriteBundle>()
                            .insert_bundle(block.sprite_bundle());
                    }
                }
            }
        }
    }
}
