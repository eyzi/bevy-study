use crate::core::camera;
use crate::core::config;
use crate::core::icon;
use crate::core::state;
use crate::fader;
use crate::menu;
use crate::splashscreen;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_ecs_ldtk::utils::translation_to_grid_coords;
use bevy_inspector_egui::WorldInspectorPlugin;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Wall;

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct WallBundle {
    wall: Wall,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct PortalIn;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct PortalOut;

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct PortalInBundle {
    portal: PortalIn,
}

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct PortalOutBundle {
    portal: PortalOut,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct StartSpot;

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct StartSpotBundle {
    start_spot: StartSpot,
}

pub fn start() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    window: WindowDescriptor {
                        width: config::WINDOW_WIDTH,
                        height: config::WINDOW_HEIGHT,
                        title: config::GAME_NAME.to_string(),
                        resizable: false,
                        present_mode: bevy::window::PresentMode::AutoVsync,
                        ..default()
                    },
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(LdtkPlugin)
        .insert_resource(LevelSelection::Index(1))
        .register_ldtk_int_cell::<WallBundle>(1)
        .register_ldtk_int_cell::<PortalOutBundle>(2)
        .register_ldtk_int_cell::<StartSpotBundle>(3)
        .register_ldtk_int_cell::<PortalInBundle>(4)
        .add_plugin(menu::plugin::MenuPlugin)
        .add_plugin(fader::plugin::FaderPlugin)
        .add_plugin(splashscreen::plugin::SplashscreenPlugin)
        .add_state(state::GameState::Startup)
        .add_startup_system(icon::setup)
        .add_startup_system(camera::setup)
        // .add_startup_system(fade_to_splashscreen)
        .add_startup_system(setup_map)
        .add_system(move_player)
        .add_system(update_sprite.after(move_player))
        .add_system(follow_player.after(move_player))
        .run();
}

#[derive(Component)]
pub struct Player {
    facing: u8,
    is_moving: bool,
}

#[derive(Resource)]
pub struct SpriteIndex {
    index: u8,
    timer: Timer,
}

impl Default for SpriteIndex {
    fn default() -> Self {
        Self {
            index: 0u8,
            timer: Timer::from_seconds(1. / 8., TimerMode::Repeating),
        }
    }
}

fn setup_map(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut textures: ResMut<Assets<TextureAtlas>>,
) {
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("rpg/map.ldtk"),
        ..Default::default()
    });

    let texture_handle = asset_server.load("rpg/Characters/Character 1.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(24.0, 32.1), 5, 8, None, None);
    let texture_atlas_handle = textures.add(texture_atlas);

    commands.spawn((
        Player {
            facing: 1,
            is_moving: false,
        },
        SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(0),
            texture_atlas: texture_atlas_handle.clone(),
            transform: Transform::from_translation(Vec3::new(135.0, 35.0, 10.0)),
            ..default()
        },
    ));
}

fn follow_player(
    mut q_camera: Query<(&mut Transform, With<Camera>, Without<Player>)>,
    q_player: Query<(&Transform, With<Player>, Without<Camera>)>,
) {
    let (mut camera, _, _) = q_camera.single_mut();
    let (player, _, _) = q_player.single();
    camera.translation = player.translation;
}

fn move_player(
    mut q_player: Query<(&mut Transform, &mut Player)>,
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
    q_walls: Query<(&GridCoords, &Wall)>,
) {
    let (mut transform, mut player) = q_player.single_mut();

    if keys.any_pressed([KeyCode::W, KeyCode::A, KeyCode::S, KeyCode::D]) {
        let mut movement = Vec2::new(0., 0.);
        let move_by = 100. * time.delta_seconds();

        if keys.pressed(KeyCode::W) {
            movement.y += 1.;
        }
        if keys.pressed(KeyCode::S) {
            movement.y -= 1.;
        }
        if keys.pressed(KeyCode::D) {
            movement.x += 1.;
        }
        if keys.pressed(KeyCode::A) {
            movement.x -= 1.;
        }

        player.is_moving = movement.y != 0f32 || movement.x != 0f32;
        if player.is_moving {
            movement = movement.normalize();
        }

        if movement.y < 0f32 {
            player.facing = 0;
        } else if movement.y > 0f32 {
            player.facing = 1;
        } else if movement.x > 0f32 {
            player.facing = 2;
        } else if movement.x < 0f32 {
            player.facing = 3;
        }

        let new_point = Vec2::new(
            transform.translation.x + (movement.x * move_by),
            transform.translation.y + (movement.y * move_by),
        );
        let new_point_coords = translation_to_grid_coords(
            Vec2::new(new_point.x, new_point.y + 2.),
            IVec2::new(16, 16),
        );

        let mut colliding = false;
        for (wall_coords, _) in q_walls.iter() {
            if wall_coords.x == new_point_coords.x && wall_coords.y == new_point_coords.y {
                colliding = true;
            }
        }

        if !colliding {
            transform.translation.x = new_point.x;
            transform.translation.y = new_point.y;
        } else {
            player.is_moving = false;
        }
    } else {
        player.is_moving = false;
    }
}

fn update_sprite(
    mut q_player: Query<(&mut TextureAtlasSprite, &Player)>,
    mut sprite_index: Local<SpriteIndex>,
    time: Res<Time>,
) {
    let (mut sprite, player) = q_player.single_mut();

    if sprite_index.timer.just_finished() {
        sprite_index.index += 1;
        sprite_index.index %= 4;
        sprite_index.timer.reset();
    } else {
        sprite_index.timer.tick(time.delta());
    }

    let mut sprite_texture_row = player.facing;
    if player.is_moving {
        sprite_texture_row += 4;
    }

    let sprite_texture_index = ((5u8 * sprite_texture_row) + sprite_index.index) as usize;
    *sprite = TextureAtlasSprite::new(sprite_texture_index);
}

fn fade_to_splashscreen(mut commands: Commands) {
    fader::plugin::create(
        &mut commands,
        1.,
        Color::WHITE,
        state::GameState::Splashscreen,
    );
}
