use bevy::prelude::*;

pub fn setup(mut commands: Commands) {
    commands.spawn().insert_bundle(Camera2dBundle::default());
}
