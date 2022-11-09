use super::super::core::state;
use bevy::prelude::*;

pub struct StartupPlugin;

impl Plugin for StartupPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(state::GameState::Startup).with_system(add_splashscreen),
        )
        .add_system_set(
            SystemSet::on_exit(state::GameState::Startup).with_system(remove_splashscreen),
        );
    }
}

fn add_splashscreen(mut _app_state: ResMut<State<state::GameState>>) {}

fn remove_splashscreen() {}
