use super::super::core::config;
use super::cell::*;
use super::tetromino::*;
use bevy::prelude::*;
use bevy::sprite::Anchor;

#[derive(Component, Clone)]
pub struct Grid {
    origin_x: i16,
    origin_y: i16,
    pub cells: Vec<Vec<Cell>>,
    pub held: Option<Tetromino>,
    pub falling: Option<Tetromino>,
    pub upcoming: Vec<Tetromino>,
}

const BLANK_COLOR: Color = Color::rgb(0.06, 0.06, 0.06);
const BORDER_COLOR: Color = Color::rgb(0.4, 0.4, 0.4);

impl Grid {
    pub fn new() -> Self {
        let columns = (config::WINDOW_WIDTH / config::CELL_SIZE) as usize;
        let rows = (config::WINDOW_HEIGHT / config::CELL_SIZE) as usize;
        let origin_x = -config::HALF_WINDOW_WIDTH as i16;
        let origin_y = -config::HALF_WINDOW_HEIGHT as i16;

        let mut grid = Grid {
            origin_x,
            origin_y,
            cells: vec![],
            held: Some(Tetromino::new(TetrominoShape::I)),
            falling: None,
            upcoming: vec![
                Tetromino::new(TetrominoShape::S),
                Tetromino::new(TetrominoShape::J),
                Tetromino::new(TetrominoShape::T),
            ],
        };

        let is_border = grid.set_border();

        let mut column_cells = vec![];
        for x in 0..columns {
            let mut row_cells: Vec<Cell> = vec![];
            for y in 0..rows {
                row_cells.push(Cell::at(x, y).set(if is_border(x, y) {
                    BORDER_COLOR
                } else {
                    BLANK_COLOR
                }));
            }
            column_cells.push(row_cells);
        }
        grid.cells = column_cells;

        grid
    }

    fn bottom() -> usize {
        ((config::WINDOW_HEIGHT / config::CELL_SIZE - 20.) / 2. - 1.) as usize
    }

    fn left() -> usize {
        ((config::WINDOW_WIDTH / config::CELL_SIZE - 10.) / 2. - 1.) as usize
    }

    fn top() -> usize {
        Self::bottom() + 21
    }

    fn right() -> usize {
        Self::left() + 11
    }

    fn held_x(&self) -> usize {
        Self::left() - 5
    }

    fn held_y(&self) -> usize {
        Self::top() - 3
    }

    fn upcoming_x(&self) -> usize {
        Self::right() + 2
    }

    fn upcoming_y(&self, index: usize) -> usize {
        Self::top() - 3 - (index as i16 * 4) as usize
    }

    fn set_border(&self) -> fn(usize, usize) -> bool {
        |x: usize, y: usize| {
            ((y >= Self::bottom() && y <= Self::top()) && (x == Self::left() || x == Self::right()))
                || ((x >= Self::left() && x <= Self::right())
                    && (y == Self::top() || y == Self::bottom()))
        }
    }

    fn within_border(&self) -> fn(usize, usize) -> bool {
        |x: usize, y: usize| {
            x > Self::left() && x < Self::right() && y > Self::bottom() && y < Self::bottom()
        }
    }
}

pub fn setup(mut commands: Commands) {
    commands.spawn().insert(Grid::new());
}

pub fn refresh(mut q: Query<&Grid>) {
    for grid in q.iter_mut() {
        // tetris. go through every row inside border. if not empty or not full, copy
        // set falling cells on position
        // set held cells
        // set upcoming cells
    }
}

pub fn draw(mut commands: Commands, mut q: Query<&Grid>) {
    for grid in q.iter_mut() {
        let origin_x = grid.origin_x;
        let origin_y = grid.origin_y;

        for (_x, rows) in grid.cells.iter().enumerate() {
            for (_y, cell) in rows.iter().enumerate() {
                cell.draw(&mut commands, origin_x, origin_y);
            }
        }

        if let Some(held) = &grid.held {
            for (x, row) in held.cells().iter().enumerate() {
                for (y, &content) in row.iter().enumerate() {
                    Cell {
                        x: grid.held_x() + x,
                        y: grid.held_y() + y,
                        content,
                    }
                    .draw(&mut commands, origin_x, origin_y);
                }
            }
        }

        for (index, upcoming) in grid.upcoming.iter().enumerate() {
            for (x, row) in upcoming.cells().iter().enumerate() {
                for (y, &content) in row.iter().enumerate() {
                    Cell {
                        x: grid.upcoming_x() + x,
                        y: grid.upcoming_y(index) + y,
                        content,
                    }
                    .draw(&mut commands, origin_x, origin_y);
                }
            }
        }
    }
}
