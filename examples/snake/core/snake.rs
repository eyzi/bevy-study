use super::app;
use super::cell;
use bevy::prelude::*;

const HEAD_COLOR: Color = Color::rgb(0.7, 0.7, 0.7);
const COLOR: Color = Color::rgb(0.3, 0.3, 0.3);

#[derive(Component)]
pub struct Snake {
    direction: Direction,
    upcoming_direction: Direction,
}

#[derive(Component, Clone, Copy, Default)]
pub struct SnakeSegment {
    pub position: cell::Position,
}

#[derive(Resource, Default, Deref, DerefMut)]
pub struct SnakeSegments(Vec<Entity>);

#[derive(Resource, Default)]
pub struct LastSnakeSegment(SnakeSegment);

#[derive(Component)]
pub struct GrowthEvent;

#[derive(PartialEq, Default, Copy, Clone)]
enum Direction {
    #[default]
    North,
    West,
    East,
    South,
}

pub fn spawn_segment(mut commands: Commands, position: cell::Position, is_head: bool) -> Entity {
    let color = if is_head { HEAD_COLOR } else { COLOR };
    let size = if is_head {
        Vec3::new(cell::SIZE, cell::SIZE, cell::SIZE)
    } else {
        Vec3::new(cell::SIZE - 5., cell::SIZE - 5., cell::SIZE - 5.)
    };

    let mut segment = commands.spawn((
        SnakeSegment { position },
        SpriteBundle {
            sprite: Sprite { color, ..default() },
            transform: Transform {
                scale: size,
                translation: position.to_pixels(),
                ..default()
            },
            ..default()
        },
    ));

    if is_head {
        segment.insert(Snake {
            direction: Direction::North,
            upcoming_direction: Direction::North,
        });
    }

    segment.id()
}

pub fn spawn(commands: Commands, mut segments: ResMut<SnakeSegments>) {
    *segments = SnakeSegments(vec![spawn_segment(
        commands,
        cell::Position { x: 0, y: 0 },
        true,
    )]);
}

pub fn r#move(
    mut head: Query<(&mut Snake, &mut SnakeSegment, &mut Transform)>,
    mut body: Query<(&mut SnakeSegment, &mut Transform, Without<Snake>)>,
    mut last_snake_segment: ResMut<LastSnakeSegment>,
    mut game_over_writer: EventWriter<app::GameOverEvent>,
) {
    if let Some((mut head, mut head_segment, mut transform)) = head.iter_mut().next() {
        // record last position
        let mut last_position = head_segment.position;

        // set the head to next position
        head.direction = head.upcoming_direction;
        if let Some((x_change, y_change)) = speeds_by_direction(head.direction) {
            head_segment.position.x += x_change;
            head_segment.position.y += y_change;

            if head_segment.position.x < -15 {
                head_segment.position.x = 15
            }
            if head_segment.position.x > 15 {
                head_segment.position.x = -15
            }
            if head_segment.position.y < -15 {
                head_segment.position.y = 15
            }
            if head_segment.position.y > 15 {
                head_segment.position.y = -15
            }

            transform.translation = head_segment.position.to_pixels();
        }

        for (mut tail_segment, mut tail_transform, _) in body.iter_mut() {
            let new_last_position = tail_segment.position;
            tail_segment.position = last_position;

            if tail_segment.position == head_segment.position {
                game_over_writer.send(app::GameOverEvent);
            }

            tail_transform.translation = tail_segment.position.to_pixels();
            last_position = new_last_position;
        }

        last_snake_segment.0.position = last_position
    }
}

pub fn grow(
    commands: Commands,
    last_segment: Res<LastSnakeSegment>,
    mut segments: ResMut<SnakeSegments>,
    mut growth_reader: EventReader<GrowthEvent>,
) {
    if growth_reader.iter().next().is_some() {
        segments.push(spawn_segment(commands, last_segment.0.position, false))
    }
}

pub fn buffer_move(mut query: Query<&mut Snake>, input: Res<Input<KeyCode>>) {
    for mut player in query.iter_mut() {
        if player.direction == Direction::North || player.direction == Direction::South {
            if input.just_pressed(KeyCode::A) {
                player.upcoming_direction = Direction::West;
            }
            if input.just_pressed(KeyCode::D) {
                player.upcoming_direction = Direction::East;
            }
        }
        if player.direction == Direction::West || player.direction == Direction::East {
            if input.just_pressed(KeyCode::W) {
                player.upcoming_direction = Direction::North;
            }
            if input.just_pressed(KeyCode::S) {
                player.upcoming_direction = Direction::South;
            }
        }
    }
}

fn speeds_by_direction(direction: Direction) -> Option<(i32, i32)> {
    if direction == Direction::North {
        return Some((0, 1));
    }
    if direction == Direction::South {
        return Some((0, -1));
    }
    if direction == Direction::West {
        return Some((-1, 0));
    }
    if direction == Direction::East {
        return Some((1, 0));
    }
    None
}
