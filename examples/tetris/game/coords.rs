use bevy::prelude::*;

#[derive(Component, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Coords {
    pub x: i8,
    pub y: i8,
}
