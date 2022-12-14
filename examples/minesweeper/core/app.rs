use super::board::options::BoardOptions;
use super::board::plugin::MinesweeperBoard;
use super::config;
use bevy::prelude::*;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    InGame,
    Out,
}

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
        .insert_resource(BoardOptions {
            map_size: (20, 20),
            bomb_count: 40,
            tile_padding: 3.,
            ..default()
        })
        .add_plugin(MinesweeperBoard {
            running_state: AppState::InGame,
        })
        .add_state(AppState::Out)
        .add_startup_system(setup_camera)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
