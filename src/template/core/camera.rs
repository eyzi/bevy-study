use bevy::core_pipeline::fxaa::Fxaa;
use bevy::prelude::*;

pub fn setup(mut commands: Commands) {
    let mut camera_bundle = Camera2dBundle::default();
    camera_bundle.camera.hdr = true;
    commands.spawn((camera_bundle, Fxaa::default()));
}
