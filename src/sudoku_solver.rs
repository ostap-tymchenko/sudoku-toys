// comfy tables for displaying, see post_board function
use comfy_table::{modifiers::UTF8_ROUND_CORNERS, presets::UTF8_FULL, *};
// random for generating a random number for a tile
use rand::prelude::*;
//HashSet is used in todo
use std::collections::HashSet;

enum BoxRange {
    TopLeft,
    TopMiddle,
    TopRight,
    MiddleLeft,
    MiddleMiddle,
    MiddleRight,
    BottomLeft,
    BottomMiddle,
    BottomRight,
}

pub fn sudoku_solver_main() {
    let board = [
        [0, 0, 0, 0, 0, 9, 0, 0, 0], // 01-09
        [0, 0, 0, 0, 0, 0, 0, 0, 4], // 10-18
        [0, 0, 7, 6, 4, 8, 0, 0, 2], // 19-27
        [0, 0, 2, 0, 5, 0, 8, 0, 0], // 28-36
        [0, 0, 1, 8, 0, 0, 0, 2, 3], // 46-45
        [7, 0, 4, 0, 0, 0, 0, 0, 0], // 46-54
        [0, 0, 0, 3, 0, 0, 0, 4, 0], // 55-63
        [0, 0, 0, 0, 6, 0, 5, 0, 0], // 64-72
        [0, 2, 3, 0, 1, 0, 0, 0, 0], // 73-81
    ];

    solve_board_brute_force(board);
}
// this takes a 9*9 array as board and prints it into a table
fn post_board(board: [[i32; 9]; 9]) {
    let mut post = Table::new();
    for i in 1..10 {
        post.add_row(board[i]);
    }
    post.load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS);

    println!("{post}");
}

// this is the initial that takes a board and asigns it a random value
// the value is then checked for validity
fn solve_board_brute_force(mut board: [[i32; 9]; 9]) -> [[i32; 9]; 9] {
    let mut tile_pos:i32 = 0;
    for rows in &board {
        for mut tile in rows {
            tile_pos += 1;
            *tile = rand::thread_rng().gen_range(1..10);
            check_legality(&mut tile, tile_pos, board);
        }
    }
    post_board(board);
    board
}

fn check_legality(tile: &mut i32, tile_pos:i32, board: [[i32; 9]; 9]) -> i32 {
    let tile_pos_f32 =f32::from(tile_pos as u8);
    dbg!(tile_pos);

    let row: [i32; 9] = board[[tile_pos_f32.into()/9.0].iter().collect()];
    dbg!(row);

    let col: [i32; 9] = board[[tile_pos_f32%9.0].iter().collect()];
    dbg!(col);
    
    // let illegal_box: [i32; 9] = 
    let x_cord = row.iter().map(|x| x/3);
    let y_cord = col.iter().map(|x| x/3);

    let xy_box = match (x_cord, y_cord) {
        (0, 0) => BoxRange::TopLeft,
        (0, 1) => BoxRange::TopMiddle,
        (0, 2) => BoxRange::TopRight,
        (1, 0) => BoxRange::MiddleLeft,
        (1, 1) => BoxRange::MiddleMiddle,
        (1, 2) => BoxRange::MiddleRight,
        (2, 0) => BoxRange::BottomLeft,
        (2, 1) => BoxRange::BottomMiddle,
        (2, 2) => BoxRange::BottomRight,
        _ => panic!("bad math"),
    };

    let illegal_combined: HashSet<_> = row
        .into_iter()
        .chain(col.into_iter())
        .chain(xy_box.iter())
        .collect();

    if illegal_combined.into().any(|x| x == *tile) {
        *tile = rand::thread_rng().gen_range(1..10);
        check_legality(tile, tile_pos, board);
        println!("legal didn't pass")
    } else {
        println!("legal passed")
    }
    *tile
}