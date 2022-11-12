use super::cell;
use super::snake;
use bevy::prelude::*;
use rand::prelude::random;

#[derive(Resource, Default)]
pub struct FoodInfo {
    pub count: i32,
}

#[derive(Component)]
pub struct Food {
    position: cell::Position,
}

const COLOR: Color = Color::rgb(1.0, 0.0, 1.0);

pub fn spawn(mut commands: Commands, mut info: ResMut<FoodInfo>) {
    if info.count > 0 {
        return;
    }

    let random_position = cell::Position {
        x: ((random::<f32>() * 31.) - 15.) as i32,
        y: ((random::<f32>() * 31.) - 15.) as i32,
    };

    commands.spawn((
        Food {
            position: random_position,
        },
        SpriteBundle {
            sprite: Sprite {
                color: COLOR,
                ..default()
            },
            transform: Transform {
                translation: random_position.to_pixels(),
                scale: Vec3::new(cell::SIZE, cell::SIZE, cell::SIZE),
                ..default()
            },
            ..default()
        },
    ));

    info.count += 1;
}

pub fn eaten(
    mut commands: Commands,
    mut info: ResMut<FoodInfo>,
    mut growth_writer: EventWriter<snake::GrowthEvent>,
    snake_query: Query<(&snake::Snake, &snake::SnakeSegment)>,
    food_query: Query<(Entity, &Food)>,
) {
    for (_, head) in snake_query.iter() {
        for (food_entity, food) in food_query.iter() {
            if food.position == head.position {
                commands.entity(food_entity).despawn();
                growth_writer.send(snake::GrowthEvent);
                info.count -= 1;
            }
        }
    }
}
