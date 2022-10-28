use super::cell::*;
use super::tetromino::*;
use bevy::prelude::*;

#[derive(Component, Clone)]
pub struct TetrominoInPlay {
    pub tetromino: Tetromino,
    pub x: i8,
    pub y: i8,
}

impl TetrominoInPlay {
    pub fn get_cells(self) -> Vec<CellInGrid> {
        let mut cells = self.tetromino.cells;
        // rotate if needed
        let mut cells_in_grid = vec![];

        for (column, cell_row) in cells.iter_mut().enumerate() {
            for (row, &mut option_cell) in cell_row.iter_mut().enumerate() {
                if let Some(cell) = option_cell {
                    let x = self.x + (column as i8);
                    let y = self.y + (row as i8);
                    cells_in_grid.push(CellInGrid { cell, x, y });
                }
            }
        }

        cells_in_grid
    }
}

#[derive(Component, Clone)]
pub struct Tetris {
    wall: Vec<Vec<Option<Cell>>>,
    falling: Vec<TetrominoInPlay>,
    upcoming: Vec<Tetromino>,
}

impl Tetris {
    pub fn blank() -> Tetris {
        Tetris {
            wall: vec![vec![None; 10]; 20],
            falling: vec![],
            upcoming: vec![
                Tetromino::new(TetrominoShape::I),
                Tetromino::new(TetrominoShape::I),
                Tetromino::new(TetrominoShape::I),
            ],
        }
    }
}

pub fn setup(mut commands: Commands) {
    commands.spawn().insert(Tetris::blank());
}
