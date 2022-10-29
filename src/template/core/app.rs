use super::config;
use super::icon;
use bevy::prelude::*;

pub fn start() {
    App::new()
        .insert_resource(WindowDescriptor {
            width: config::WINDOW_WIDTH,
            height: config::WINDOW_HEIGHT,
            title: config::GAME_NAME.to_string(),
            resizable: false,
            present_mode: bevy::window::PresentMode::AutoVsync,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(icon::setup)
        .run();
}
