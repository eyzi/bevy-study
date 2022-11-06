use bevy::prelude::*;

use super::block::Block;
use super::coords::Coords;
use super::screen::{border_left, border_right};
use super::tetromino::*;

#[derive(Component)]
pub struct Collidable;

pub fn has_collision(
    tetromino: &Tetromino,
    collidable_query: &mut Query<&Block, With<Collidable>>,
    origin_x: i8,
    origin_y: i8,
) -> bool {
    for block in collidable_query.iter_mut() {
        for (x, row) in tetromino.cells().into_iter().enumerate() {
            for (y, cell) in row.into_iter().enumerate() {
                if let Some(_color) = cell {
                    if origin_x + x as i8 <= border_left() || origin_x + x as i8 >= border_right() {
                        return true;
                    }

                    let cell_coords = Coords {
                        x: origin_x + x as i8,
                        y: origin_y + y as i8,
                    };
                    if block.coords.eq(&cell_coords) {
                        return true;
                    }
                }
            }
        }
    }
    false
}
