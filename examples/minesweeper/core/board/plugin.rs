use super::super::coordinates::Coordinates;
use super::super::tile::*;
use super::super::tile_map::TileMap;
use super::options::{BoardOptions, BoardPosition, TileSize};
use bevy::ecs::schedule::StateData;
use bevy::prelude::*;

pub struct MinesweeperBoard<T> {
    pub running_state: T,
}

impl<T: StateData> Plugin for MinesweeperBoard<T> {
    fn build(&self, app: &mut App) {
        app.add_startup_system(Self::create_board);
    }
}

impl<T> MinesweeperBoard<T> {
    pub fn create_board(
        mut commands: Commands,
        board_options: Option<Res<BoardOptions>>,
        asset_server: Res<AssetServer>,
    ) {
        let font: Handle<Font> = asset_server.load("minesweeper/font.ttf");
        let bomb_image: Handle<Image> = asset_server.load("minesweeper/bomb.png");

        let options = match board_options {
            None => BoardOptions::default(),
            Some(o) => o.clone(),
        };

        let mut tile_map = TileMap::empty(options.map_size.0, options.map_size.1);
        tile_map.set_bombs(options.bomb_count);

        let tile_size = match options.tile_size {
            TileSize::Fixed(v) => v,
            TileSize::Adaptive { min, max } => {
                Self::adaptive_tile_size((min, max), (tile_map.width, tile_map.height))
            }
        };

        let board_size = Vec2::new(
            tile_map.width as f32 * tile_size,
            tile_map.height as f32 * tile_size,
        );

        let board_position = match options.position {
            BoardPosition::Custom(p) => p,
            BoardPosition::Centered { offset } => {
                Vec3::new(-(board_size.x / 2.), -(board_size.y / 2.), 0.) + offset
            }
        };

        commands
            .spawn(Name::new("Board"))
            .insert(Transform::from_translation(board_position))
            .insert(GlobalTransform::default())
            .with_children(|parent| {
                parent
                    .spawn(SpriteBundle {
                        sprite: Sprite {
                            color: Color::WHITE,
                            custom_size: Some(board_size),
                            ..default()
                        },
                        transform: Transform::from_xyz(board_size.x / 2., board_size.y / 2., 0.),
                        ..default()
                    })
                    .insert(Name::new("Background"));

                Self::spawn_tiles(
                    parent,
                    &tile_map,
                    tile_size,
                    options.tile_padding,
                    Color::GRAY,
                    bomb_image,
                    font,
                );
            });
    }

    fn adaptive_tile_size((min, max): (f32, f32), (width, height): (u16, u16)) -> f32 {
        // let max_width = window.width / width as f32;
        // let max_height = window.height / height as f32;
        // max_width.min(max_height).clamp(min, max)
        1.
    }

    fn bomb_count_text_bundle(count: u8, font: Handle<Font>, size: f32) -> Text2dBundle {
        let (text, color) = (
            count.to_string(),
            match count {
                1 => Color::WHITE,
                2 => Color::GREEN,
                3 => Color::YELLOW,
                4 => Color::ORANGE,
                _ => Color::PURPLE,
            },
        );

        Text2dBundle {
            text: Text {
                sections: vec![TextSection {
                    value: text,
                    style: TextStyle {
                        color,
                        font,
                        font_size: size,
                    },
                }],
                alignment: TextAlignment {
                    vertical: VerticalAlign::Center,
                    horizontal: HorizontalAlign::Center,
                },
            },
            transform: Transform::from_xyz(0., 0., 1.),
            ..default()
        }
    }

    fn spawn_tiles(
        parent: &mut ChildBuilder,
        tile_map: &TileMap,
        size: f32,
        padding: f32,
        color: Color,
        bomb_image: Handle<Image>,
        font: Handle<Font>,
    ) {
        for (y, line) in tile_map.iter().enumerate() {
            for (x, tile) in line.iter().enumerate() {
                let coords = Coordinates {
                    x: x as u16,
                    y: y as u16,
                };
                let mut commands = parent.spawn((
                    SpriteBundle {
                        sprite: Sprite {
                            color,
                            custom_size: Some(Vec2::splat(size - padding)),
                            ..default()
                        },
                        transform: Transform::from_xyz(
                            (x as f32 * size) + (size / 2.),
                            (y as f32 * size) + (size / 2.),
                            1.,
                        ),
                        ..default()
                    },
                    Name::new(format!("Tile ({}, {})", x, y)),
                    coords,
                ));

                match tile {
                    Tile::Bomb => {
                        commands.insert(Bomb).with_children(|parent| {
                            parent.spawn_bundle(SpriteBundle {
                                sprite: Sprite {
                                    custom_size: Some(Vec2::splat(size - padding)),
                                    ..default()
                                },
                                transform: Transform::from_xyz(0., 0., 1.),
                                texture: bomb_image.clone(),
                                ..default()
                            });
                        });
                    }
                    Tile::BombNeighbor(v) => {
                        commands
                            .insert(BombNeighbor { count: *v })
                            .with_children(|parent| {
                                parent.spawn(Self::bomb_count_text_bundle(
                                    *v,
                                    font.clone(),
                                    size - padding,
                                ));
                            });
                    }
                    Tile::Empty => (),
                }
            }
        }
    }
}
