use super::super::core::config;
use bevy::prelude::*;
use bevy::sprite::Anchor;

#[derive(Component, Clone, Copy)]
pub struct Cell {
    pub x: usize,
    pub y: usize,
    pub content: Option<Color>,
}

#[derive(Component, Clone, Copy)]
pub struct BorderCell(Cell);

impl Cell {
    pub fn to_raw_coords(cell_x: usize, cell_y: usize, origin_x: i16, origin_y: i16) -> (i16, i16) {
        let raw_x = origin_x + (config::CELL_SIZE as i16 * cell_x as i16);
        let raw_y = origin_y + (config::CELL_SIZE as i16 * cell_y as i16);
        (raw_x, raw_y)
    }

    pub fn at(x: usize, y: usize) -> Self {
        Cell {
            x,
            y,
            content: None,
        }
    }

    pub fn set(mut self, color: Color) -> Self {
        self.content = Some(color);
        self
    }

    #[allow(dead_code)]
    pub fn clear(&mut self) {
        self.content = None;
    }

    pub fn draw(self, commands: &mut Commands, origin_x: i16, origin_y: i16) {
        let (x_translation, y_translation) =
            Self::to_raw_coords(self.x, self.y, origin_x, origin_y);

        if let Some(color) = self.content {
            commands.spawn().insert(self).insert_bundle(SpriteBundle {
                sprite: Sprite {
                    color,
                    anchor: Anchor::BottomLeft,
                    ..default()
                },
                transform: Transform {
                    scale: Vec3::new(
                        config::CELL_SIZE - config::CELL_MARGIN,
                        config::CELL_SIZE - config::CELL_MARGIN,
                        1.,
                    ),
                    translation: Vec3::new(x_translation as f32, y_translation as f32, 0.),
                    ..default()
                },
                ..default()
            });
        }
    }
}

pub fn clear(mut commands: Commands, q: Query<Entity, With<Cell>>) {
    for cell in q.iter() {
        commands.entity(cell).despawn();
    }
}
