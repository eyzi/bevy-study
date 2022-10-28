use bevy::prelude::*;

#[derive(Component, Clone)]
pub struct CellInGrid {
    pub cell: Cell,
    pub x: i8,
    pub y: i8,
}

#[derive(Component, Debug, Clone, Copy)]
pub struct Cell {
    pub color: Color,
}

impl Cell {
    pub fn new(color: Color) -> Self {
        Cell { color }
    }
}
