use bevy::prelude::*;

#[derive(Component)]
struct Player {
    delta_x: f32,
}

#[derive(Component)]
struct Laser;

#[derive(Component, Clone, Copy)]
enum EnemyMovement {
    Left,
    Right,
    Down { n: f32, next_left: bool },
}

#[derive(Component)]
struct Enemy {
    movement: EnemyMovement,
}

const WIDTH: f32 = 1280.0;
const HEIGHT: f32 = 720.0;

const X_MARGIN: f32 = 00.0;
const Y_MARGIN: f32 = 50.0;
const MAX_X_MOVEMENT: f32 = (WIDTH / 2.0) - X_MARGIN;
const ACCELERATION: f32 = 100.0;
const MAX_VELOCITY: f32 = 1600.0;
const ENEMY_LATERAL_SPEED: f32 = 100.0;

pub fn start() {
    App::new()
        .insert_resource(ClearColor(Color::MIDNIGHT_BLUE))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                width: WIDTH,
                height: HEIGHT,
                title: "Space Invade.rs".to_string(),
                resizable: false,
                present_mode: bevy::window::PresentMode::AutoVsync,
                ..Default::default()
            },
            ..default()
        }))
        .add_startup_system(setup)
        .add_system(player)
        .add_system(enemy_movement)
        .add_system(laser_movement)
        .add_system(enemy_zapper)
        .run();
}

fn setup(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut textures: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = assets.load("space-invaders/spritesheet.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(24.0, 24.0), 3, 1, None, None);
    let texture_atlas_handle = textures.add(texture_atlas);

    commands.spawn(Camera2dBundle::default());
    commands
        .spawn(Player { delta_x: 0.0 })
        .insert(SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(0),
            texture_atlas: texture_atlas_handle.clone(),
            transform: Transform::from_translation(Vec3::new(
                0.0,
                -((HEIGHT / 2.0) - Y_MARGIN),
                0.0,
            )),
            ..default()
        });

    for enemy_row in 0..4 {
        let y = 200.0 - (enemy_row as f32 * 30.0);
        for enemy_column in 0..20 {
            let x = 200.0 - (enemy_column as f32 * 30.0);
            let movement = if enemy_row % 2 == 0 {
                EnemyMovement::Left
            } else {
                EnemyMovement::Right
            };

            commands
                .spawn(Enemy { movement })
                .insert(SpriteSheetBundle {
                    sprite: TextureAtlasSprite::new(1),
                    texture_atlas: texture_atlas_handle.clone(),
                    transform: Transform::from_translation(Vec3::new(x, y, 0.0)),
                    ..Default::default()
                });
        }
    }
}

fn player(
    time: Res<Time>,
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Player, &mut Transform, &Handle<TextureAtlas>)>,
) {
    for (mut player, mut transform, atlas_handle) in query.iter_mut() {
        let mut firing = false;

        if keyboard_input.pressed(KeyCode::A) {
            player.delta_x -= ACCELERATION;
        }
        if keyboard_input.pressed(KeyCode::D) {
            player.delta_x += ACCELERATION;
        }
        if keyboard_input.just_pressed(KeyCode::Space) {
            firing = true;
        }

        // Apply movement deltas
        player.delta_x = player.delta_x.clamp(-MAX_VELOCITY, MAX_VELOCITY);
        transform.translation.x += player.delta_x * time.delta_seconds();
        transform.translation.x = transform
            .translation
            .x
            .clamp(-MAX_X_MOVEMENT, MAX_X_MOVEMENT);

        if firing {
            commands.spawn(Laser).insert(SpriteSheetBundle {
                texture_atlas: atlas_handle.clone(),
                transform: Transform::from_translation(Vec3::new(
                    transform.translation.x,
                    transform.translation.y,
                    0.0,
                )),
                sprite: TextureAtlasSprite::new(2),
                ..Default::default()
            });
        }

        player.delta_x *= 0.75;
    }
}

fn enemy_movement(time: Res<Time>, mut query: Query<(&mut Enemy, &mut Transform)>) {
    for (mut enemy, mut transform) in query.iter_mut() {
        let mut new_movement = enemy.movement;
        match enemy.movement {
            EnemyMovement::Left => {
                transform.translation.x -= ENEMY_LATERAL_SPEED * time.delta_seconds();
                if transform.translation.x < -400.0 {
                    new_movement = EnemyMovement::Down {
                        n: 1.0,
                        next_left: false,
                    }
                }
            }
            EnemyMovement::Right => {
                transform.translation.x += ENEMY_LATERAL_SPEED * time.delta_seconds();
                if transform.translation.x > 400.0 {
                    new_movement = EnemyMovement::Down {
                        n: 1.0,
                        next_left: true,
                    }
                }
            }
            EnemyMovement::Down { n, next_left } => {
                transform.translation.y -= 15.0;
                new_movement = EnemyMovement::Down {
                    n: n - 1.0,
                    next_left,
                };
                if n < 1.0 {
                    new_movement = if next_left {
                        EnemyMovement::Left
                    } else {
                        EnemyMovement::Right
                    }
                }
            }
        }

        enemy.movement = new_movement;
    }
}

fn laser_movement(
    time: Res<Time>,
    mut query: Query<(Entity, &Laser, &mut Transform)>,
    mut commands: Commands,
) {
    let y = 1000.0 * time.delta_seconds();
    for (entity, _, mut trans) in query.iter_mut() {
        trans.translation += Vec3::new(0.0, y, 0.0);

        if trans.translation.y > 300.0 {
            commands.entity(entity).despawn();
        }
    }
}

fn enemy_zapper(
    laser_query: Query<(Entity, &Laser, &Transform)>,
    collider_query: Query<(Entity, &Enemy, &Transform)>,
    mut commands: Commands,
) {
    for (laser, _, trans) in laser_query.iter() {
        let laser_pos = Vec2::new(trans.translation.x, trans.translation.y);
        for (enemy, _, enemy_transform) in collider_query.iter() {
            let enemy_pos = Vec2::new(enemy_transform.translation.x, enemy_transform.translation.y);

            if enemy_pos.distance(laser_pos) < 24.0 {
                commands.entity(enemy).despawn();
                commands.entity(laser).despawn();
            }
        }
    }
}
