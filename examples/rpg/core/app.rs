use super::super::menu;
use super::super::startup;
use super::camera;
use super::config;
use super::icon;
use super::state;
use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;

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
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(startup::plugin::StartupPlugin)
        .add_plugin(menu::plugin::MainMenuPlugin)
        .add_state(state::GameState::MainMenu)
        .add_startup_system(icon::setup)
        .add_startup_system(camera::setup)
        .run();
}
