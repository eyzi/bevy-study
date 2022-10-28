use super::cell::Cell;
use super::config;
use super::tetris::*;
use super::tetromino::*;
use bevy::prelude::*;
use bevy::sprite::Anchor;

#[derive(Component, Clone)]
pub struct Grid(Vec<Vec<Cell>>);

const BLANK_COLOR: Color = Color::rgb(0.06, 0.06, 0.06);

impl Grid {
    pub fn new() -> Self {
        return Grid(vec![
            vec![
                Cell::new(BLANK_COLOR);
                (config::WINDOW_WIDTH / config::CELL_SIZE) as usize
            ];
            (config::WINDOW_HEIGHT / config::CELL_SIZE) as usize
        ]);
    }

    pub fn set_cell(&mut self, x: usize, y: usize, cell: Cell) {
        let cells = &mut self.0;
        cells[x][y] = cell;
    }

    pub fn bottom() -> usize {
        (((config::WINDOW_HEIGHT / config::CELL_SIZE) - config::GRID_HEIGHT as f32) / 2. - 1.)
            as usize
    }

    pub fn left() -> usize {
        (((config::WINDOW_WIDTH / config::CELL_SIZE) - config::GRID_WIDTH as f32) / 2. - 1.)
            as usize
    }

    pub fn top() -> usize {
        Grid::bottom() + config::GRID_HEIGHT as usize + 1
    }

    pub fn right() -> usize {
        Grid::left() + config::GRID_WIDTH as usize + 1
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

    let grid_bottom = Grid::bottom();
    let grid_top = Grid::top();
    let grid_left = Grid::left();
    let grid_right = Grid::right();

    println!("grid_bottom: {}, grid_top: {}", grid_bottom, grid_top);
    println!("grid_left: {}, grid_right: {}", grid_left, grid_right);

    for y in grid_bottom..=grid_top {
        grid.set_cell(grid_left, y, border_cell);
        grid.set_cell(grid_right, y, border_cell);
    }

    for x in grid_left..=grid_right {
        grid.set_cell(x, grid_bottom, border_cell);
        grid.set_cell(x, grid_top, border_cell);
    }
}

pub fn draw(
    mut commands: Commands,
    mut grid_query: Query<&mut Grid>,
    mut tetris_query: Query<&mut Tetris>,
) {
    let mut grid = grid_query.iter_mut().next().unwrap();
    let mut tetris = tetris_query.iter_mut().next().unwrap();

    for (x, rows) in grid.0.iter_mut().enumerate() {
        for (y, cell) in rows.iter().enumerate() {
            commands
                .spawn()
                .insert(cell.clone())
                .insert_bundle(cell.draw(x, y));
        }
    }

    let upcoming_x = Grid::right() as i8 + 4;
    let mut upcoming_y = Grid::top().clone() as i8;

    for tetromino in tetris.clone().upcoming {
        let shape = TetrominoInPlace {
            tetromino,
            x: upcoming_x,
            y: upcoming_y,
        };

        upcoming_y -= 5;

        for gridcell in shape.get_cells().iter_mut() {
            commands
                .spawn()
                .insert_bundle(gridcell.cell.draw(gridcell.x as usize, gridcell.y as usize));
        }
    }

    if let Some(held) = tetris.clone().hold {
        let hold_x = Grid::left() as i8 - 3;
        let hold_y = Grid::top() as i8;

        let shape = TetrominoInPlace {
            tetromino: held,
            x: hold_x,
            y: hold_y,
        };

        for gridcell in shape.get_cells().iter_mut() {
            commands
                .spawn()
                .insert_bundle(gridcell.cell.draw(gridcell.x as usize, gridcell.y as usize));
        }
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
