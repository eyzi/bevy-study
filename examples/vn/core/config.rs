use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;

pub const GAME_NAME: &str = "Eyzi's VN";
pub const WINDOW_WIDTH: f32 = 1280.;
pub const WINDOW_HEIGHT: f32 = 720.;
pub const GAME_FONT_PATH: &str = "common/font.ttf";

#[allow(dead_code)]
#[derive(Resource)]
pub struct GameConfig {
    pub game_name: String,
    pub window_width: f32,
    pub window_height: f32,
    pub game_font: Handle<Font>,
}

pub fn add_default_plugins() -> PluginGroupBuilder {
    DefaultPlugins.set(WindowPlugin {
        window: WindowDescriptor {
            width: WINDOW_WIDTH,
            height: WINDOW_HEIGHT,
            title: GAME_NAME.to_string(),
            resizable: false,
            present_mode: bevy::window::PresentMode::AutoVsync,
            ..default()
        },
        ..default()
    })
}

pub fn setup(mut commands: Commands, asset_server: ResMut<AssetServer>) {
    commands.insert_resource(ClearColor(Color::WHITE));
    commands.insert_resource(GameConfig {
        game_name: GAME_NAME.to_string(),
        window_width: WINDOW_WIDTH,
        window_height: WINDOW_HEIGHT,
        game_font: asset_server.load(GAME_FONT_PATH),
    });
}
