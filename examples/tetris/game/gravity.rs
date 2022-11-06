use super::block::*;
use super::collission::*;
use super::coords::*;
use super::screen::*;
use super::tetromino::{clear as clear_falling, *};
use bevy::prelude::*;

#[derive(Component)]
pub struct Falling {
    pub coords: Coords,
}

pub fn apply(
    mut commands: Commands,
    grid_query: Query<&mut Grid>,
    mut collidable_query: Query<&Block, With<Collidable>>,
    mut falling_query: Query<(Entity, &mut Tetromino, &mut Falling), Without<Upcoming>>,
) {
    let grid = grid_query.single();
    if !falling_query.is_empty() {
        let (falling_entity, tetromino, mut falling) = falling_query.single_mut();
        if !has_collision(
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
        } else {
            // check game over
            persist_tetromino(
                &mut commands,
                grid,
                tetromino.clone(),
                falling.coords.x,
                falling.coords.y,
            );
            commands.entity(falling_entity).despawn();
        }
    }
}

pub fn create_falling(commands: &mut Commands, tetromino: Tetromino) {
    let coords = Coords {
        x: border_left() + ((border_right() - border_left()) / 2),
        y: border_top(),
    };

    commands
        .spawn()
        .insert(Falling { coords })
        .insert(tetromino);
}
