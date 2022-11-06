use bevy::prelude::*;

#[derive(Component, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Coords {
    pub x: u8,
    pub y: u8,
}
