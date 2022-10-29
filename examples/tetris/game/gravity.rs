use super::grid::*;
use bevy::prelude::*;

pub fn apply(mut q: Query<&mut Grid>) {
    let mut grid = q.iter_mut().next().unwrap();
    if let Some(_falling) = grid.falling.clone() {
        if let Some(new_fallin_y) = grid.falling_y.checked_sub(1) {
            grid.falling_y = new_fallin_y;
        }
    }

    // if below is not a border or not colliding with another cell
}
