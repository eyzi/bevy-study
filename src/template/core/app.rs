use crate::core::camera;
use crate::core::config;
use crate::core::icon;
use crate::core::state;
use bevy::prelude::*;

pub fn start() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                width: config::WINDOW_WIDTH,
                height: config::WINDOW_HEIGHT,
                title: config::GAME_NAME.to_string(),
                resizable: false,
                present_mode: bevy::window::PresentMode::AutoVsync,
                ..default()
            },
            ..default()
        }))
        .add_state(state::GameState::Playing)
        .add_startup_system(icon::setup)
        .add_startup_system(camera::setup)
        .run();
}
