use crate::core::state;
use crate::fader::plugin::{create as create_fader, Fader};
use crate::menu::plugin;
use bevy::prelude::*;

pub struct ScenePlugin;

#[derive(Component)]
pub struct SceneScreen;

impl Plugin for ScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(state::GameState::Playing).with_system(create_scene),
        )
        .add_system_set(
            SystemSet::on_exit(state::GameState::Playing)
                .with_system(plugin::remove_menu_screen::<SceneScreen>),
        )
        .add_system_set(
            SystemSet::on_update(state::GameState::Playing).with_system(handle_interaction),
        );
    }
}

pub fn create_scene(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(SpriteBundle {
        texture: asset_server.load("vn/BG.png"),
        ..default()
    });
    commands.spawn(SpriteBundle {
        texture: asset_server.load("vn/Hana/Hana Smile.png"),
        transform: Transform {
            scale: Vec3::new(0.8, 0.8, 0.),
            translation: Vec3::new(0., -40., 0.),
            ..default()
        },
        ..default()
    });
}

pub fn handle_interaction(
    mut commands: Commands,
    mouse_event: Res<Input<MouseButton>>,
    mut fader_q: Query<&Fader>,
) {
    if fader_q.iter_mut().next().is_none() && mouse_event.just_pressed(MouseButton::Left) {
        create_fader(
            &mut commands,
            0.5,
            Color::BLACK,
            state::GameState::OptionsMenu,
        );
    }
}
