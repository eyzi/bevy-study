use bevy::prelude::*;
use rand::{prelude::thread_rng, Rng};
use std::cmp;
use std::time::Duration;

const MAX_SNOW: usize = 50;
const MAX_SNOW_SPAWN: usize = 10;
const SNOW_MIN_SPEED: f32 = 5.;
const SNOW_MAX_SPEED: f32 = 30.;

#[derive(Default, Component, Reflect)]
#[reflect(Component)]
pub struct Snow {
    speed: Vec2,
    life: Timer,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum SnowState {
    Snowing,
    NoSnow,
}

#[derive(Resource)]
pub struct SnowSpawnTimer(Timer);

impl Default for SnowSpawnTimer {
    fn default() -> Self {
        Self(Timer::new(Duration::from_secs(1), TimerMode::Once))
    }
}

pub struct SnowPlugin;

impl Plugin for SnowPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Snow>()
            .add_state(SnowState::Snowing)
            .add_system_set(
                SystemSet::on_update(SnowState::Snowing)
                    .with_system(spawn_snow)
                    .with_system(snowfall),
            )
            .add_system_set(SystemSet::on_update(SnowState::NoSnow).with_system(snowfall_off));
    }
}

fn spawn_snow(
    mut commands: Commands,
    q_snow: Query<&Snow>,
    mut snow_spawn_timer: Local<SnowSpawnTimer>,
    time: Res<Time>,
) {
    if snow_spawn_timer.0.just_finished() {
        let snow_count = q_snow.iter().count();
        if snow_count < MAX_SNOW {
            let spawnable_count = cmp::min(MAX_SNOW - snow_count, MAX_SNOW_SPAWN);
            for _ in 1..thread_rng().gen_range(1..spawnable_count) {
                let x = thread_rng().gen_range(-50f32..300f32);
                let y = thread_rng().gen_range(50f32..400f32);

                let random_x_speed = thread_rng().gen_range(-5f32..5f32);
                let random_y_speed = thread_rng().gen_range(SNOW_MIN_SPEED..SNOW_MAX_SPEED);

                commands.spawn((
                    Snow {
                        speed: Vec2::new(random_x_speed, random_y_speed),
                        life: Timer::new(Duration::from_secs(10), TimerMode::Once),
                    },
                    SpriteBundle {
                        sprite: Sprite {
                            color: Color::rgba(1., 1., 1., 0.0).into(),
                            custom_size: Some(Vec2::new(3., 3.)),
                            ..default()
                        },
                        transform: Transform::from_xyz(x, y, 30.),
                        ..default()
                    },
                ));
            }
        }

        snow_spawn_timer.0 = Timer::new(Duration::from_secs(1), TimerMode::Once);
    } else {
        snow_spawn_timer.0.tick(time.delta());
    }
}

fn snowfall(
    mut commands: Commands,
    mut q_snow: Query<(Entity, &mut Transform, &mut Snow, &mut Sprite)>,
    time: Res<Time>,
) {
    for (entity, mut transform, mut snow, mut sprite) in q_snow.iter_mut() {
        if snow.life.just_finished() {
            commands.entity(entity).despawn_recursive();
        } else {
            if snow.life.percent() < 0.05 {
                let enter_opacity = snow.life.percent() / 0.05;
                sprite.color.set_a(enter_opacity);
            } else if snow.life.percent_left() < 0.05 {
                let exit_opacity = snow.life.percent_left() / 0.05;
                sprite.color.set_a(exit_opacity);
            }
            transform.translation.x += snow.speed.x * time.delta_seconds();
            transform.translation.y -= snow.speed.y * time.delta_seconds();
            snow.life.tick(time.delta());
        }
    }
}

fn snowfall_off(
    mut commands: Commands,
    mut q_snow: Query<(Entity, &mut Transform, &mut Snow, &mut Sprite)>,
    time: Res<Time>,
) {
    for (entity, mut transform, snow, mut sprite) in q_snow.iter_mut() {
        if sprite.color.a() <= 0. {
            commands.entity(entity).despawn_recursive();
        } else {
            let new_a = sprite.color.a() - (0.1 * time.delta_seconds());
            sprite.color.set_a(new_a);
            transform.translation.x += snow.speed.x * time.delta_seconds();
            transform.translation.y -= snow.speed.y * time.delta_seconds();
        }
    }
}
