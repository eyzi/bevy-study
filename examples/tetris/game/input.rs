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
    mut falling_query: Query<(Entity, &mut Tetromino, &mut Falling)>,
    mut held_query: Query<(Entity, &mut Tetromino, With<Held>, Without<Falling>)>,
    upcoming_query: Query<(
        Entity,
        &mut Tetromino,
        &mut Upcoming,
        Without<Falling>,
        Without<Held>,
    )>,
) {
    let grid = grid_query.single();

    if !falling_query.is_empty() {
        let (falling_entity, mut tetromino, mut falling) = falling_query.single_mut();
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
                falling.coords.x -= 1;
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
                falling.coords.x += 1;
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
                falling.coords.y -= 1;
            }
        }
        if keyboard_input.just_pressed(KeyCode::Z) {
            clear_falling(
                &mut commands,
                grid,
                &tetromino,
                falling.coords.x,
                falling.coords.y,
            );
            if let Some((held_entity, held_tetromino, _, _)) = held_query.iter_mut().next() {
                commands.entity(falling_entity).despawn();
                commands.entity(held_entity).despawn();
                create_falling(&mut commands, held_tetromino.clone());
                create_held(&mut commands, tetromino.clone());
            } else {
                commands.entity(falling_entity).despawn();
                create_held(&mut commands, tetromino.clone());
            }
        }
    }
}
