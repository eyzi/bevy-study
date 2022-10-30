use bevy::prelude::*;

pub fn spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Load all the meshes
    let king_handle: Handle<Mesh> = asset_server.load("chess/pieces.glb#Mesh0/Primitive0");
    let king_cross_handle: Handle<Mesh> = asset_server.load("chess/pieces.glb#Mesh1/Primitive0");
    let pawn_handle: Handle<Mesh> = asset_server.load("chess/pieces.glb#Mesh2/Primitive0");
    let knight_1_handle: Handle<Mesh> = asset_server.load("chess/pieces.glb#Mesh3/Primitive0");
    let knight_2_handle: Handle<Mesh> = asset_server.load("chess/pieces.glb#Mesh4/Primitive0");
    let rook_handle: Handle<Mesh> = asset_server.load("chess/pieces.glb#Mesh5/Primitive0");
    let bishop_handle: Handle<Mesh> = asset_server.load("chess/pieces.glb#Mesh6/Primitive0");
    let queen_handle: Handle<Mesh> = asset_server.load("chess/pieces.glb#Mesh7/Primitive0");

    // Add some materials
    let white_material = materials.add(Color::rgb(1., 1., 1.).into());
    let black_material = materials.add(Color::rgb(0.06, 0.06, 0.06).into());

    spawn_king(
        &mut commands,
        king_handle.clone(),
        king_cross_handle.clone(),
        white_material.clone(),
        Vec3::new(0., 0., 4.),
    );
    spawn_queen(
        &mut commands,
        queen_handle.clone(),
        white_material.clone(),
        Vec3::new(0., 0., 3.),
    );
    spawn_bishop(
        &mut commands,
        bishop_handle.clone(),
        white_material.clone(),
        Vec3::new(0., 0., 2.),
    );
    spawn_bishop(
        &mut commands,
        bishop_handle.clone(),
        white_material.clone(),
        Vec3::new(0., 0., 5.),
    );
    spawn_knight(
        &mut commands,
        knight_1_handle.clone(),
        knight_2_handle.clone(),
        white_material.clone(),
        Vec3::new(0., 0., 6.),
        false,
    );
    spawn_knight(
        &mut commands,
        knight_1_handle.clone(),
        knight_2_handle.clone(),
        white_material.clone(),
        Vec3::new(0., 0., 1.),
        false,
    );
    spawn_rook(
        &mut commands,
        rook_handle.clone(),
        white_material.clone(),
        Vec3::new(0., 0., 0.),
    );
    spawn_rook(
        &mut commands,
        rook_handle.clone(),
        white_material.clone(),
        Vec3::new(0., 0., 7.),
    );

    spawn_king(
        &mut commands,
        king_handle.clone(),
        king_cross_handle.clone(),
        black_material.clone(),
        Vec3::new(7., 0., 4.),
    );
    spawn_queen(
        &mut commands,
        queen_handle.clone(),
        black_material.clone(),
        Vec3::new(7., 0., 3.),
    );
    spawn_bishop(
        &mut commands,
        bishop_handle.clone(),
        black_material.clone(),
        Vec3::new(7., 0., 2.),
    );
    spawn_bishop(
        &mut commands,
        bishop_handle.clone(),
        black_material.clone(),
        Vec3::new(7., 0., 5.),
    );
    spawn_knight(
        &mut commands,
        knight_1_handle.clone(),
        knight_2_handle.clone(),
        black_material.clone(),
        Vec3::new(7., 0., 6.),
        true,
    );
    spawn_knight(
        &mut commands,
        knight_1_handle.clone(),
        knight_2_handle.clone(),
        black_material.clone(),
        Vec3::new(7., 0., 1.),
        true,
    );
    spawn_rook(
        &mut commands,
        rook_handle.clone(),
        black_material.clone(),
        Vec3::new(7., 0., 0.),
    );
    spawn_rook(
        &mut commands,
        rook_handle.clone(),
        black_material.clone(),
        Vec3::new(7., 0., 7.),
    );

    for i in 0..8 {
        spawn_pawn(
            &mut commands,
            pawn_handle.clone(),
            white_material.clone(),
            Vec3::new(1., 0., i as f32),
        );
        spawn_pawn(
            &mut commands,
            pawn_handle.clone(),
            black_material.clone(),
            Vec3::new(6., 0., i as f32),
        );
    }
}

fn spawn_king(
    commands: &mut Commands,
    mesh: Handle<Mesh>,
    mesh_cross: Handle<Mesh>,
    material: Handle<StandardMaterial>,
    position: Vec3,
) {
    commands
        .spawn()
        .insert_bundle(PbrBundle {
            transform: Transform::from_translation(position),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn().insert_bundle(PbrBundle {
                mesh,
                material: material.clone(),
                transform: {
                    let mut transform = Transform::from_translation(Vec3::new(-0.2, 0., -1.9));
                    transform.apply_non_uniform_scale(Vec3::new(0.2, 0.2, 0.2));
                    transform
                },
                ..default()
            });
            parent.spawn().insert_bundle(PbrBundle {
                mesh: mesh_cross,
                material: material.clone(),
                transform: {
                    let mut transform = Transform::from_translation(Vec3::new(-0.2, 0., -1.9));
                    transform.apply_non_uniform_scale(Vec3::new(0.2, 0.2, 0.2));
                    transform
                },
                ..default()
            });
        });
}

fn spawn_queen(
    commands: &mut Commands,
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
    position: Vec3,
) {
    commands
        .spawn()
        .insert_bundle(PbrBundle {
            transform: Transform::from_translation(position),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn().insert_bundle(PbrBundle {
                mesh,
                material,
                transform: {
                    let mut transform = Transform::from_translation(Vec3::new(-0.2, 0., -0.95));
                    transform.apply_non_uniform_scale(Vec3::new(0.2, 0.2, 0.2));
                    transform
                },
                ..default()
            });
        });
}

fn spawn_bishop(
    commands: &mut Commands,
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
    position: Vec3,
) {
    commands
        .spawn()
        .insert_bundle(PbrBundle {
            transform: Transform::from_translation(position),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn().insert_bundle(PbrBundle {
                mesh,
                material,
                transform: {
                    let mut transform = Transform::from_translation(Vec3::new(-0.1, 0., 0.));
                    transform.apply_non_uniform_scale(Vec3::new(0.2, 0.2, 0.2));
                    transform
                },
                ..default()
            });
        });
}

fn spawn_knight(
    commands: &mut Commands,
    mesh_1: Handle<Mesh>,
    mesh_2: Handle<Mesh>,
    material: Handle<StandardMaterial>,
    position: Vec3,
    is_black: bool,
) {
    let rotate_angle = if is_black { 3.14 } else { 0. };
    let translation = if is_black {
        Vec3::new(0., 0., -0.85)
    } else {
        Vec3::new(-0.2, 0., 0.9)
    };

    commands
        .spawn()
        .insert_bundle(PbrBundle {
            transform: Transform::from_translation(position),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn().insert_bundle(PbrBundle {
                mesh: mesh_1,
                material: material.clone(),
                transform: {
                    let mut transform = Transform::from_translation(Vec3::new(-0.2, 0., 0.9));
                    transform.apply_non_uniform_scale(Vec3::new(0.2, 0.2, 0.2));
                    transform //.looking_at(Vec3::new(0., 0., 0.), Vec3::new(0., 0., 0.))
                },
                ..default()
            });
            parent.spawn().insert_bundle(PbrBundle {
                mesh: mesh_2,
                material: material.clone(),
                transform: {
                    let mut transform = Transform::from_translation(translation);
                    transform.apply_non_uniform_scale(Vec3::new(0.2, 0.2, 0.2));
                    transform.rotate_y(rotate_angle);
                    transform //.looking_at(Vec3::new(0., 0., 0.), Vec3::new(0., 0., 0.))
                },
                ..default()
            });
        });
}

fn spawn_rook(
    commands: &mut Commands,
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
    position: Vec3,
) {
    commands
        .spawn()
        .insert_bundle(PbrBundle {
            transform: Transform::from_translation(position),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn().insert_bundle(PbrBundle {
                mesh,
                material,
                transform: {
                    let mut transform = Transform::from_translation(Vec3::new(-0.1, 0., 1.8));
                    transform.apply_non_uniform_scale(Vec3::new(0.2, 0.2, 0.2));
                    transform
                },
                ..default()
            });
        });
}

fn spawn_pawn(
    commands: &mut Commands,
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
    position: Vec3,
) {
    commands
        .spawn()
        .insert_bundle(PbrBundle {
            transform: Transform::from_translation(position),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn().insert_bundle(PbrBundle {
                mesh,
                material,
                transform: {
                    let mut transform = Transform::from_translation(Vec3::new(-0.2, 0., 2.6));
                    transform.apply_non_uniform_scale(Vec3::new(0.2, 0.2, 0.2));
                    transform
                },
                ..default()
            });
        });
}
