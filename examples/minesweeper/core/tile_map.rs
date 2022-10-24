use super::coordinates::Coordinates;
use super::tile::Tile;
use rand::{thread_rng, Rng};
use std::ops::{Deref, DerefMut};

#[derive(Debug, Clone)]
pub struct TileMap {
    pub bomb_count: u16,
    pub height: u16,
    pub width: u16,
    pub map: Vec<Vec<Tile>>,
}

const SQUARE_COORDINATES: [(i8, i8); 8] = [
    // Bottom left
    (-1, -1),
    // Bottom
    (0, -1),
    // Bottom right
    (1, -1),
    // Left
    (-1, 0),
    // Right
    (1, 0),
    // Top Left
    (-1, 1),
    // Top
    (0, 1),
    // Top right
    (1, 1),
];

impl TileMap {
    pub fn empty(width: u16, height: u16) -> Self {
        let map = (0..height)
            .into_iter()
            .map(|_| (0..width).into_iter().map(|_| Tile::Empty).collect())
            .collect();
        Self {
            bomb_count: 0,
            height,
            width,
            map,
        }
    }

    pub fn safe_square_at(&self, coords: Coordinates) -> impl Iterator<Item = Coordinates> {
        SQUARE_COORDINATES
            .iter()
            .copied()
            .map(move |tuple| coords + tuple)
    }

    pub fn is_bomb_at(&self, coords: Coordinates) -> bool {
        if coords.x >= self.width || coords.y >= self.height {
            return false;
        };

        self.map[coords.y as usize][coords.x as usize].is_bomb()
    }

    pub fn bomb_count_at(&self, coords: Coordinates) -> u8 {
        if self.is_bomb_at(coords) {
            return 0;
        }
        self.safe_square_at(coords)
            .filter(|coord| self.is_bomb_at(*coord))
            .count() as u8
    }

    pub fn set_bombs(&mut self, bomb_count: u16) {
        self.bomb_count = bomb_count;
        let mut remaining_bombs = bomb_count;
        let mut rng = thread_rng();

        while remaining_bombs > 0 {
            let (x, y) = (
                rng.gen_range(0..self.width) as usize,
                rng.gen_range(0..self.height) as usize,
            );
            if let Tile::Empty = self[x][y] {
                self[x][y] = Tile::Bomb;
                remaining_bombs -= 1;
            }
        }

        for y in 0..self.height {
            for x in 0..self.width {
                let coords = Coordinates { x, y };
                if self.is_bomb_at(coords) {
                    continue;
                }

                let num = self.bomb_count_at(coords);
                if num == 0 {
                    continue;
                }

                let tile = &mut self[y as usize][x as usize];
                *tile = Tile::BombNeighbor(num);
            }
        }
    }
}

impl Deref for TileMap {
    type Target = Vec<Vec<Tile>>;

    fn deref(&self) -> &Self::Target {
        &self.map
    }
}

impl DerefMut for TileMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.map
    }
}
