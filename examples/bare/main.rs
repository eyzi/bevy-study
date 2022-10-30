#![windows_subsystem = "windows"]

use bevy::prelude::{App, Camera2dBundle, Commands, DefaultPlugins};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup_camera)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn().insert_bundle(Camera2dBundle::default());
}
