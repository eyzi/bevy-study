use super::grid::*;
use bevy::prelude::*;

pub fn handle(keyboard_input: Res<Input<KeyCode>>, mut q: Query<&mut Grid>) {
    let mut grid = q.iter_mut().next().unwrap();

    // get new x and y if can turn

    if let Some(falling) = &mut grid.falling {
        if keyboard_input.just_pressed(KeyCode::Q) {
            falling.rotate_anticlockwise();
        }
        if keyboard_input.just_pressed(KeyCode::E) {
            falling.rotate_clockwise();
        }
        if keyboard_input.just_pressed(KeyCode::A) {
            if let Some(new_falling_x) = grid.falling_x.checked_sub(1) {
                grid.falling_x = new_falling_x;
            }
        }
        if keyboard_input.just_pressed(KeyCode::D) {
            if let Some(new_falling_x) = grid.falling_x.checked_add(1) {
                grid.falling_x = new_falling_x;
            }
        }
        if keyboard_input.just_pressed(KeyCode::S) {
            if let Some(new_falling_y) = grid.falling_y.checked_sub(1) {
                grid.falling_y = new_falling_y;
            }
        }
    }
}
