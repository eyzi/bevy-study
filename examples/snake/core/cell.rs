use bevy::prelude::*;

pub const SIZE: f32 = 20.;

#[derive(Component, Clone, Copy, PartialEq, Eq, Default)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn to_pixels(self: Self) -> Vec3 {
        Vec3::new(self.x as f32 * SIZE, self.y as f32 * SIZE, 0.0)
    }
}
