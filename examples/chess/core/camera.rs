use bevy::prelude::*;
// use bevy_mod_picking::*;

pub fn setup(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_matrix(Mat4::from_rotation_translation(
            Quat::from_xyzw(-0.3, -0.5, -0.3, 0.5).normalize(),
            Vec3::new(-2.7, 10.5, 3.5),
        )),
        ..default()
    });
    // .insert_bundle(PickingCameraBundle::default());
}
