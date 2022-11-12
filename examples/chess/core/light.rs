use bevy::prelude::*;

pub fn setup(mut commands: Commands) {
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 3000.,
            radius: 50.,
            range: 100.,
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(4.5, 4.5, 4.0)),
        ..default()
    });
}
