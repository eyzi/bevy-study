use bevy::prelude::*;

use super::block::Block;
use super::coords::Coords;
use super::tetromino::*;

#[derive(Component)]
pub struct Collidable;

pub fn has_collission(
    tetromino: &Tetromino,
    collidable_query: &mut Query<&Block, With<Collidable>>,
    origin_x: u8,
    origin_y: u8,
) -> bool {
    for block in collidable_query.iter_mut() {
        for (x, row) in tetromino.cells().into_iter().enumerate() {
            for (y, cell) in row.into_iter().enumerate() {
                if let Some(_color) = cell {
                    let cell_coords = Coords {
                        x: origin_x + x as u8,
                        y: origin_y + y as u8,
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
