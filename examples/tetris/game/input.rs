use super::block::*;
use super::collission::*;
use super::gravity::*;
use super::screen::*;
use super::tetromino::{clear as clear_falling, *};
use bevy::prelude::*;

pub fn handle(
    mut commands: Commands,
    mut collidable_query: Query<&Block, With<Collidable>>,
    keyboard_input: Res<Input<KeyCode>>,
    grid_query: Query<&mut Grid>,
    mut falling_query: Query<(&mut Tetromino, &mut Falling)>,
) {
    let grid = grid_query.single();

    for (mut tetromino, mut falling) in falling_query.iter_mut() {
        if keyboard_input.just_pressed(KeyCode::Q) {
            let mut mock_tetromino = tetromino.clone();
            mock_tetromino.rotate_anticlockwise();
            if !has_collission(
                &mock_tetromino,
                &mut collidable_query,
                falling.coords.x,
                falling.coords.y,
            ) {
                clear_falling(
                    &mut commands,
                    grid,
                    &tetromino,
                    falling.coords.x,
                    falling.coords.y,
                );
                tetromino.rotate_anticlockwise();
            }
        }
        if keyboard_input.just_pressed(KeyCode::E) {
            let mut mock_tetromino = tetromino.clone();
            mock_tetromino.rotate_clockwise();
            if !has_collission(
                &mock_tetromino,
                &mut collidable_query,
                falling.coords.x,
                falling.coords.y,
            ) {
                clear_falling(
                    &mut commands,
                    grid,
                    &tetromino,
                    falling.coords.x,
                    falling.coords.y,
                );
                tetromino.rotate_clockwise();
            }
        }
        if keyboard_input.just_pressed(KeyCode::A) {
            if !has_collission(
                &tetromino,
                &mut collidable_query,
                falling.coords.x - 1,
                falling.coords.y,
            ) {
                clear_falling(
                    &mut commands,
                    grid,
                    &tetromino,
                    falling.coords.x,
                    falling.coords.y,
                );
                falling.coords.x = falling.coords.x.checked_sub(1).unwrap_or(falling.coords.x);
            }
        }
        if keyboard_input.just_pressed(KeyCode::D) {
            if !has_collission(
                &tetromino,
                &mut collidable_query,
                falling.coords.x + 1,
                falling.coords.y,
            ) {
                clear_falling(
                    &mut commands,
                    grid,
                    &tetromino,
                    falling.coords.x,
                    falling.coords.y,
                );
                falling.coords.x = falling.coords.x.checked_add(1).unwrap_or(falling.coords.x);
            }
        }
        if keyboard_input.just_pressed(KeyCode::S) {
            if !has_collission(
                &tetromino,
                &mut collidable_query,
                falling.coords.x,
                falling.coords.y - 1,
            ) {
                clear_falling(
                    &mut commands,
                    grid,
                    &tetromino,
                    falling.coords.x,
                    falling.coords.y,
                );
                falling.coords.y = falling.coords.y.checked_sub(1).unwrap_or(falling.coords.y);
            }
        }
    }
}
