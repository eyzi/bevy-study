use super::super::game::board;
use super::super::game::pieces;
use super::camera;
use super::config;
use super::icon;
use super::light;
use bevy::prelude::*;
use bevy_mod_picking::*;

pub fn start() {
    App::new()
        .insert_resource(ClearColor(Color::TEAL))
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(WindowDescriptor {
            width: config::WINDOW_WIDTH,
            height: config::WINDOW_HEIGHT,
            title: config::GAME_NAME.to_string(),
            resizable: false,
            present_mode: bevy::window::PresentMode::AutoVsync,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugins(DefaultPickingPlugins)
        .add_startup_system(camera::setup)
        .add_startup_system(icon::setup)
        .add_startup_system(light::setup)
        .add_startup_system(board::spawn)
        .add_startup_system(pieces::spawn)
        .run();
}
