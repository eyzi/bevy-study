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

pub fn clear(
    mut commands: Commands,
    grid_query: Query<&mut Grid>,
    mut falling_query: Query<(&Tetromino, &Falling)>,
) {
    let grid = grid_query.single();

    if !falling_query.is_empty() {
        for (tetromino, falling) in falling_query.iter_mut() {
            clear_falling(
                &mut commands,
                grid,
                &tetromino,
                falling.coords.x,
                falling.coords.y,
            );
        }
    }
}

pub fn apply(
    mut commands: Commands,
    mut collidable_query: Query<&Block, With<Collidable>>,
    mut falling_query: Query<(Entity, &mut Tetromino, &mut Falling), Without<Upcoming>>,
    mut upcoming_query: Query<(Entity, &mut Tetromino, &mut Upcoming), Without<Falling>>,
) {
    if !falling_query.is_empty() {
        let (falling_entity, tetromino, mut falling) = falling_query.single_mut();
        if has_collission(
            &tetromino,
            &mut collidable_query,
            falling.coords.x,
            falling.coords.y - 1,
        ) {
            commands.entity(falling_entity).despawn();
        } else {
            falling.coords.y = falling.coords.y.checked_sub(1).unwrap_or(falling.coords.y);
        }
    } else {
        for (upcoming_entity, tetromino, mut upcoming) in upcoming_query.iter_mut() {
            if (upcoming.index == 0) {
                create_falling(&mut commands, tetromino.clone());
                commands.entity(upcoming_entity).despawn();
            } else {
                upcoming.index -= 1;
            }
            create_upcoming(&mut commands, 2, Tetromino::new(random_shape()));
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
