use super::super::menu;
use super::super::startup;
use super::camera;
use super::config;
use super::fader;
use super::icon;
use super::state;
use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;

pub fn start() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(ClearColor(Color::BLACK))
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
        .add_plugin(menu::plugin::MenuPlugin)
        .add_state(state::GameState::Splashscreen)
        // .add_startup_system(fade_to_splashscreen)
        .add_startup_system(icon::setup)
        .add_startup_system(camera::setup)
        .add_system(fader::handle)
        .run();
}

fn fade_to_splashscreen(mut commands: Commands) {
    fader::create(
        &mut commands,
        1.,
        Color::BLUE,
        state::GameState::Splashscreen,
    );
}
