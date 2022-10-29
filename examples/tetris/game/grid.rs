use super::super::core::config;
use super::cell::*;
use super::tetromino::*;
use bevy::prelude::*;

#[derive(Component, Clone)]
pub struct Grid {
    origin_x: i16,
    origin_y: i16,
    pub static_cells: Vec<Vec<Cell>>,
    pub cells: Vec<Vec<Cell>>,
    pub held: Option<Tetromino>,
    pub falling: Option<Tetromino>,
    pub falling_x: usize,
    pub falling_y: usize,
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
            static_cells: vec![vec![]],
            cells: vec![vec![]],
            held: Some(Tetromino::new(TetrominoShape::I)),
            falling: Some(Tetromino::new(TetrominoShape::Z)),
            falling_x: Grid::falling_x_start(),
            falling_y: Grid::falling_y_start(),
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
        grid.static_cells = column_cells;

        grid
    }

    fn bottom() -> usize {
        ((config::WINDOW_HEIGHT / config::CELL_SIZE - config::GRID_HEIGHT as f32) / 2. - 1.)
            as usize
    }

    fn left() -> usize {
        ((config::WINDOW_WIDTH / config::CELL_SIZE - config::GRID_WIDTH as f32) / 2. - 1.) as usize
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

    fn falling_x_start() -> usize {
        Self::left() + (config::GRID_WIDTH / 2) as usize - 1
    }

    fn falling_y_start() -> usize {
        Self::top()
    }

    fn set_border(&self) -> fn(usize, usize) -> bool {
        |x: usize, y: usize| {
            ((y >= Self::bottom() && y <= Self::top()) && (x == Self::left() || x == Self::right()))
                || ((x >= Self::left() && x <= Self::right())
                    && (y == Self::top() || y == Self::bottom()))
        }
    }

    fn within_grid(&self, x: usize, y: usize) -> bool {
        x < self.cells.len() as usize && y < self.cells[0].len()
    }

    fn within_border(&self, x: usize, y: usize) -> bool {
        x > Self::left() && x < Self::right() && y > Self::bottom() && y < Self::top()
    }
}

pub fn setup(mut commands: Commands) {
    commands.spawn().insert(Grid::new());
}

pub fn refresh(mut q: Query<&mut Grid>) {
    // for grid in q.iter_mut() {
    //      tetris. go through every row inside border. if not empty or not full, copy
    //      set falling cells on position
    //      set held cells
    //      set upcoming cells
    // }

    let mut grid = q.iter_mut().next().unwrap();

    grid.cells = grid.static_cells.clone();

    if let Some(falling) = grid.falling.clone() {
        for (falling_y, row) in falling.cells().iter().enumerate() {
            for (falling_x, &content) in row.iter().enumerate() {
                let x = grid.falling_x + falling_x;
                let y = grid.falling_y + falling_y;
                if grid.within_border(x, y) {
                    grid.cells[x][y] = grid.cells[x][y].set(if let Some(color) = content {
                        color
                    } else {
                        BLANK_COLOR
                    });
                }
            }
        }
    }

    if let Some(held) = grid.held.clone() {
        for (held_x, row) in held.cells().iter().enumerate() {
            for (held_y, &content) in row.iter().enumerate() {
                let x = grid.held_x() + held_x;
                let y = grid.held_y() + held_y;
                if grid.within_grid(x, y) {
                    grid.cells[x][y] = grid.cells[x][y].set(if let Some(color) = content {
                        color
                    } else {
                        BLANK_COLOR
                    });
                }
            }
        }
    }

    for (index, upcoming) in grid.upcoming.clone().iter().enumerate() {
        for (upcoming_x, row) in upcoming.cells().iter().enumerate() {
            for (upcoming_y, &content) in row.iter().enumerate() {
                let x = grid.upcoming_x() + upcoming_x;
                let y = grid.upcoming_y(index) + upcoming_y;
                if grid.within_grid(x, y) {
                    grid.cells[x][y] = grid.cells[x][y].set(if let Some(color) = content {
                        color
                    } else {
                        BLANK_COLOR
                    });
                }
            }
        }
    }
}

pub fn draw(mut commands: Commands, mut q: Query<&Grid>) {
    let grid = q.iter_mut().next().unwrap();
    let origin_x = grid.origin_x;
    let origin_y = grid.origin_y;

    for rows in grid.cells.iter() {
        for cell in rows.iter() {
            cell.draw(&mut commands, origin_x, origin_y);
        }
    }
}
