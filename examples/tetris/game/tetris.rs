use super::block::*;
use super::screen::*;
use bevy::prelude::*;
use bevy::utils::hashbrown::HashMap;

pub fn check_tetris(mut block_query: Query<&mut Block, With<Clearable>>) {
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
        println!("row {} has {}", key, value);
        if value >= 10 {
            tetris_rows.push(key);
        }
    }

    if tetris_rows.len() > 0 {
        remove_tetris_rows(&mut block_query, tetris_rows);
    }
}

pub fn remove_tetris_rows(block_query: &mut Query<&mut Block, With<Clearable>>, rows: Vec<i8>) {
    for mut block in block_query.iter_mut() {
        if rows.contains(&block.coords.y) {
            block.color = BLANK_COLOR;
        }
    }
}
