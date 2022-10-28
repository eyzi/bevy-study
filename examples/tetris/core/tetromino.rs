use super::cell::*;
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
    All,
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

#[derive(Component, Clone)]
pub struct TetrominoInPlace {
    pub tetromino: Tetromino,
    pub x: i8,
    pub y: i8,
}

impl TetrominoInPlace {
    pub fn get_cells(self) -> Vec<CellInGrid> {
        let mut cells = self.tetromino.cells;
        // rotate if needed
        let mut cells_in_grid = vec![];

        for (column, cell_row) in cells.iter_mut().enumerate() {
            for (row, &mut option_cell) in cell_row.iter_mut().enumerate() {
                if let Some(cell) = option_cell {
                    let x = self.x + (column as i8 - 2);
                    let y = self.y + (row as i8 - 3);
                    cells_in_grid.push(CellInGrid { cell, x, y });
                }
            }
        }

        cells_in_grid
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
        TetrominoShape::All => vec![
            vec![
                Some(Cell::new(Color::RED)),
                Some(Cell::new(Color::RED)),
                Some(Cell::new(Color::RED)),
                Some(Cell::new(Color::RED)),
            ],
            vec![
                Some(Cell::new(Color::RED)),
                Some(Cell::new(Color::RED)),
                Some(Cell::new(Color::RED)),
                Some(Cell::new(Color::RED)),
            ],
            vec![
                Some(Cell::new(Color::RED)),
                Some(Cell::new(Color::RED)),
                Some(Cell::new(Color::RED)),
                Some(Cell::new(Color::RED)),
            ],
            vec![
                Some(Cell::new(Color::RED)),
                Some(Cell::new(Color::RED)),
                Some(Cell::new(Color::RED)),
                Some(Cell::new(Color::RED)),
            ],
        ],
        _ => vec![vec![None; 4]; 4],
    }
}
