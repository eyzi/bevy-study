use super::cell::*;
use super::tetromino::*;
use bevy::prelude::*;

#[derive(Component, Clone)]
pub struct Tetris {
    pub wall: Vec<Vec<Option<Cell>>>,
    pub hold: Option<Tetromino>,
    pub falling: Vec<TetrominoInPlace>,
    pub upcoming: Vec<Tetromino>,
}

impl Tetris {
    pub fn blank() -> Tetris {
        Tetris {
            wall: vec![vec![None; 10]; 20],
            hold: Some(Tetromino::new(TetrominoShape::All)),
            falling: vec![],
            upcoming: vec![
                Tetromino::new(TetrominoShape::All),
                Tetromino::new(TetrominoShape::All),
                Tetromino::new(TetrominoShape::All),
            ],
        }
    }
}

pub fn setup(mut commands: Commands) {
    commands.spawn().insert(Tetris::blank());
}
