use super::camera;
use super::cell;
use super::food;
use super::snake;
use bevy::prelude::*;
use bevy::time::*;

const FPS: f64 = 13 as f64;
const COLUMNS: i32 = 31; // must be odd
const ROWS: i32 = 31; // must be odd
const WIDTH: f32 = cell::SIZE * (COLUMNS as f32);
const HEIGHT: f32 = cell::SIZE * (ROWS as f32);

pub struct GameOverEvent;

pub fn start() {
    App::new()
        .insert_resource(ClearColor(Color::MIDNIGHT_BLUE))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                width: WIDTH,
                height: HEIGHT,
                title: "Snake".to_string(),
                resizable: false,
                present_mode: bevy::window::PresentMode::AutoVsync,
                ..Default::default()
            },
            ..default()
        }))
        .add_startup_system(camera::setup)
        .add_startup_system(snake::spawn)
        .insert_resource(snake::SnakeSegments::default())
        .insert_resource(snake::LastSnakeSegment::default())
        .insert_resource(food::FoodInfo::default())
        .add_event::<snake::GrowthEvent>()
        .add_event::<GameOverEvent>()
        .add_system(snake::buffer_move)
        .add_system(
            snake::r#move
                .with_run_criteria(FixedTimestep::steps_per_second(FPS))
                .label("snake_move"),
        )
        .add_system(food::spawn.with_run_criteria(FixedTimestep::steps_per_second(FPS)))
        .add_system(food::eaten.label("food_eaten").after("snake_move"))
        .add_system(snake::grow.after("food_eaten"))
        .add_system(game_over.after("snake_move"))
        .run();
}

fn game_over(
    mut commands: Commands,
    mut reader: EventReader<GameOverEvent>,
    mut info: ResMut<food::FoodInfo>,
    segments_res: ResMut<snake::SnakeSegments>,
    food: Query<Entity, With<food::Food>>,
    segments: Query<Entity, With<snake::SnakeSegment>>,
) {
    if reader.iter().next().is_some() {
        for ent in segments.iter() {
            commands.entity(ent).despawn();
        }

        for ent in food.iter() {
            commands.entity(ent).despawn();
            info.count -= 1;
        }

        snake::spawn(commands, segments_res);
    }
}
