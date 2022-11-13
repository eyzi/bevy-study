use crate::core::state;
use bevy::prelude::*;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(state::GameState::Playing).with_system(add_level))
            .add_system_set(SystemSet::on_exit(state::GameState::Playing).with_system(remove_level))
            .add_system_set(
                SystemSet::on_update(state::GameState::Playing).with_system(update_level),
            );
    }
}

fn add_level() {}
fn remove_level() {}
fn update_level() {}
