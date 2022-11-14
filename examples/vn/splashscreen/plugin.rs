use super::super::core::state;
use super::super::fader::plugin::{create as create_fader, Fader};
use bevy::prelude::*;

pub struct SplashscreenPlugin;

#[derive(Component)]
pub struct Splashscreen {
    timer: Timer,
}

impl Default for Splashscreen {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(2., TimerMode::Once),
        }
    }
}

impl Plugin for SplashscreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(state::GameState::Splashscreen).with_system(add_splashscreen),
        )
        .add_system_set(
            SystemSet::on_exit(state::GameState::Splashscreen).with_system(remove_splashscreen),
        )
        .add_system_set(
            SystemSet::on_update(state::GameState::Splashscreen).with_system(update_splashscreen),
        );
    }
}

fn add_splashscreen(mut commands: Commands, asset_server: ResMut<AssetServer>) {
    let texture = asset_server.load("common/eyzi-logo.png");

    commands
        .spawn((
            Splashscreen::default(),
            SpriteBundle {
                sprite: Sprite {
                    color: Color::WHITE,
                    custom_size: Some(Vec2::new(1280., 720.)),
                    ..default()
                },
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn(SpriteBundle {
                texture,
                transform: Transform {
                    scale: Vec3::new(0.5, 0.5, 1.),
                    ..default()
                },
                ..default()
            });
        });
}

fn update_splashscreen(
    mut commands: Commands,
    time: Res<Time>,
    mut splashscreen_query: Query<&mut Splashscreen>,
    buttons: Res<Input<MouseButton>>,
    mut fader_q: Query<&Fader>,
) {
    if let Some(mut splashscreen) = splashscreen_query.iter_mut().next() {
        if (buttons.just_pressed(MouseButton::Left) || splashscreen.timer.just_finished())
            && fader_q.iter_mut().next().is_none()
        {
            create_fader(&mut commands, 1., Color::BLACK, state::GameState::MainMenu);
        } else {
            splashscreen.timer.tick(time.delta());
        }
    }
}

fn remove_splashscreen(mut commands: Commands, splashscreen_query: Query<Entity, &Splashscreen>) {
    if let Some(splashscreen) = splashscreen_query.iter().next() {
        commands.entity(splashscreen).despawn_recursive();
    }
}
