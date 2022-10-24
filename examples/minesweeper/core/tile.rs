use bevy::prelude::*;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Component)]
pub struct Bomb;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Component)]
pub struct BombNeighbor {
    pub count: u8,
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Component)]
pub struct Uncover;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Tile {
    Bomb,
    BombNeighbor(u8),
    Empty,
}

impl Tile {
    pub const fn is_bomb(&self) -> bool {
        matches!(self, Self::Bomb)
    }
}
