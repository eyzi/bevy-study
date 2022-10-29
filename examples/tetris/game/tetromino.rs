use bevy::prelude::*;

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
