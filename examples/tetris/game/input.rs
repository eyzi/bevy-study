use super::grid::*;
use bevy::prelude::*;

pub fn handle(keyboard_input: Res<Input<KeyCode>>, mut q: Query<&mut Grid>) {
    // TODO: rotating held and upcoming only for testing
    for mut grid in q.iter_mut() {
        if let Some(held) = &mut grid.held {
            if keyboard_input.just_pressed(KeyCode::A) {
                held.rotate_anticlockwise();
            } else if keyboard_input.just_pressed(KeyCode::D) {
                held.rotate_clockwise();
            }
        }
        if let Some(falling) = &mut grid.falling {
            if keyboard_input.just_pressed(KeyCode::A) {
                falling.rotate_anticlockwise();
            } else if keyboard_input.just_pressed(KeyCode::D) {
                falling.rotate_clockwise();
            }
        }
        for upcoming in &mut grid.upcoming {
            if keyboard_input.just_pressed(KeyCode::A) {
                upcoming.rotate_anticlockwise();
            } else if keyboard_input.just_pressed(KeyCode::D) {
                upcoming.rotate_clockwise();
            }
        }
    }
}
