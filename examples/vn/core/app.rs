use crate::core::camera;
use crate::core::config;
use crate::core::icon;
use crate::core::state;
use crate::fader;
use crate::menu;
use crate::scene;
use crate::splashscreen;
use bevy::prelude::*;

pub fn start() {
    App::new()
        .add_plugins(config::add_default_plugins())
        .add_plugin(menu::plugin::MenuPlugin)
        .add_plugin(fader::plugin::FaderPlugin)
        .add_plugin(splashscreen::plugin::SplashscreenPlugin)
        .add_plugin(scene::plugin::ScenePlugin)
        .add_state(state::GameState::Startup)
        .add_startup_system(config::setup)
        .add_startup_system(icon::setup)
        .add_startup_system(camera::setup)
        .add_startup_system(fade_to_splashscreen)
        .run();
}

fn fade_to_splashscreen(mut commands: Commands, state: Res<State<state::GameState>>) {
    if state.current() == &state::GameState::Startup {
        fader::plugin::create(
            &mut commands,
            1.,
            Color::WHITE,
            state::GameState::Splashscreen,
        );
    }
}
