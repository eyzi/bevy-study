use crate::core::camera;
use crate::core::config;
use crate::core::icon;
use crate::core::state;
use crate::fader;
use crate::menu;
use crate::splashscreen;
use bevy::prelude::*;
// use bevy_inspector_egui::WorldInspectorPlugin;

pub fn start() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
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
        // .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(menu::plugin::MenuPlugin)
        .add_plugin(fader::plugin::FaderPlugin)
        .add_plugin(splashscreen::plugin::SplashscreenPlugin)
        .add_state(state::GameState::Startup)
        .add_startup_system(icon::setup)
        .add_startup_system(camera::setup)
        .add_startup_system(fade_to_splashscreen)
        .run();
}

fn fade_to_splashscreen(mut commands: Commands) {
    fader::plugin::create(
        &mut commands,
        1.,
        Color::BLACK,
        state::GameState::Splashscreen,
    );
}
