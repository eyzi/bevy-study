use super::cell::Cell;
use bevy::prelude::*;

#[derive(Clone, Copy)]
pub enum TetrominoShape {
    I,
    O,
    L,
    J,
    S,
    Z,
    T,
}

#[derive(Clone, Copy)]
pub enum TetrominoRotation {
    Zero,
    Ninety,
    OneEighty,
    TwoSeventy,
}

#[derive(Component, Clone)]
pub struct Tetromino {
    pub shape: TetrominoShape,
    pub cells: Vec<Vec<Option<Cell>>>,
    pub rotation: TetrominoRotation,
}

impl Tetromino {
    pub fn new(shape: TetrominoShape) -> Self {
        Tetromino {
            shape,
            cells: cells_by_shape(shape),
            rotation: TetrominoRotation::Zero,
        }
    }

    pub fn rotate_clockwise(mut self) -> Self {
        self.rotation = match self.rotation {
            TetrominoRotation::Zero => TetrominoRotation::Ninety,
            TetrominoRotation::Ninety => TetrominoRotation::OneEighty,
            TetrominoRotation::OneEighty => TetrominoRotation::TwoSeventy,
            TetrominoRotation::TwoSeventy => TetrominoRotation::Zero,
        };

        self
    }

    pub fn rotate_anticlockwise(mut self) -> Self {
        self.rotation = match self.rotation {
            TetrominoRotation::Zero => TetrominoRotation::TwoSeventy,
            TetrominoRotation::Ninety => TetrominoRotation::Zero,
            TetrominoRotation::OneEighty => TetrominoRotation::Ninety,
            TetrominoRotation::TwoSeventy => TetrominoRotation::OneEighty,
        };

        self
    }
}

fn cells_by_shape(shape: TetrominoShape) -> Vec<Vec<Option<Cell>>> {
    match shape {
        TetrominoShape::I => vec![
            vec![None, None, None, None],
            vec![
                Some(Cell::new(Color::YELLOW)),
                Some(Cell::new(Color::YELLOW)),
                Some(Cell::new(Color::YELLOW)),
                Some(Cell::new(Color::YELLOW)),
            ],
            vec![None, None, None, None],
            vec![None, None, None, None],
        ],
        _ => vec![vec![None; 4]; 4],
    }
}
