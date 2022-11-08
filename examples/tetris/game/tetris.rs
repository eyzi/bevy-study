use super::block::*;
use super::collission::Collidable;
use super::coords::*;
use super::screen::*;
use bevy::prelude::*;
use bevy::utils::hashbrown::HashMap;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    Playing,
    GameOver,
}

pub fn check_tetris(
    mut commands: Commands,
    mut grid_query: Query<&Grid>,
    mut block_query: Query<&mut Block, With<Clearable>>,
) {
    let grid = grid_query.single_mut();
    let mut tetromino_rows: HashMap<i8, i8> = HashMap::new();

    for block in block_query.iter_mut() {
        if within_border(block.coords.x as i16, block.coords.y as i16) {
            let row = block.coords.y;

            if block.color != BLANK_COLOR {
                match tetromino_rows.get_mut(&row) {
                    Some(row_value) => {
                        *row_value += 1;
                    }
                    None => {
                        tetromino_rows.insert(row, 1);
                    }
                };
            }
        }
    }

    let mut tetris_rows: Vec<i8> = vec![];
    for (&key, &value) in tetromino_rows.iter() {
        if value >= 10 {
            tetris_rows.push(key);
        }
    }

    if tetris_rows.len() > 0 {
        remove_tetris_rows(&mut commands, &grid, &mut block_query, tetris_rows);
    }
}

pub fn remove_tetris_rows(
    commands: &mut Commands,
    grid: &Grid,
    block_query: &mut Query<&mut Block, With<Clearable>>,
    rows: Vec<i8>,
) {
    let mut grid_colors: Vec<Vec<Option<Color>>> = vec![];

    for y in border_bottom() + 1..border_top() {
        if !rows.contains(&y) {
            let mut row: Vec<Option<Color>> = vec![];
            for x in border_left() + 1..border_right() {
                let block_entity = grid.blocks.get(&Coords { x, y }).unwrap();
                if let Ok(block) = block_query.get_component::<Block>(*block_entity) {
                    if block.color == BLANK_COLOR {
                        row.push(None);
                    } else {
                        row.push(Some(block.color));
                    }
                } else {
                    row.push(None);
                }
            }
            grid_colors.push(row);
        }
    }

    for y in border_bottom() + 1..border_top() {
        for x in border_left() + 1..border_right() {
            let grid_colors_y = (y - (border_bottom() + 1)) as usize;
            let grid_colors_x = (x - (border_left() + 1)) as usize;

            let empty_row: Vec<Option<Color>> = vec![];
            let row = grid_colors
                .get((grid_colors_y) as usize)
                .unwrap_or(&empty_row);
            let block_entity = grid.blocks.get(&Coords { x, y }).unwrap();
            let item = row.get(grid_colors_x).unwrap_or(&None as &Option<Color>);
            if let Some(color) = item {
                if let Ok(mut block) = block_query.get_component_mut::<Block>(*block_entity) {
                    block.color = color.clone();
                    commands
                        .entity(*block_entity)
                        .remove_bundle::<SpriteBundle>()
                        .insert_bundle(block.sprite_bundle());
                }
            } else {
                let block = Block {
                    coords: Coords { x, y },
                    color: BLANK_COLOR,
                };
                commands
                    .entity(*block_entity)
                    .remove::<Block>()
                    .remove_bundle::<SpriteBundle>()
                    .remove::<Collidable>()
                    .insert(block.clone())
                    .insert_bundle(block.sprite_bundle());
            }
        }
    }
}
