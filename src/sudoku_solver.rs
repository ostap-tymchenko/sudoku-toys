// comfy tables for displaying, see post_board function
use comfy_table::{modifiers::UTF8_ROUND_CORNERS, presets::UTF8_FULL, *};
// random for generating a random number for a tile
use rand::prelude::*;
//HashSet is used in todo
use std::collections::HashSet;
// for flooring
use math::round;

enum box_range {
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
    println!("sudoku_solver.rs init");

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
fn post_board(board: [[u8; 9]; 9]) {
    let mut post = Table::new();
    for i in 0..9 {
        post.add_row(board[i]);
    }
    post.load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS);

    println!("{post}");
}

// this is the initial that takes a board and asigns it a random value
// the value is then checked for validity
fn solve_board_brute_force(mut board: [[u8; 9]; 9]) -> [[u8; 9]; 9] {
    let mut tile_pos:u8 = 0;
    for rows in &board {
        for mut tile in rows {
            tile_pos += 1;
            *tile = rand::thread_rng().gen_range(1..=9);
            check_legality(&mut tile, tile_pos, board);
        }
    }
    post_board(board);
    board
}

fn check_legality(tile: &mut u8, tile_pos: u8, board: [[u8; 9]; 9]) -> u8 {

    let row: [u8; 9] = board[round::floor[tile_pos/9]];
    dbg!(row);

    let col: [u8; 9] = board[[tile_pos%9].into()];
    dbg!(col);
    
    // let illegal_box: [u8; 9] = 
    let x_cord = row.map(|x| x/3).collect();
    let y_cord = col.map(|x| x/3).collect();

    let illegal_combined: HashSet<_> = row
        .into_iter()
        .chain(col.into_iter())
        .chain(xy_box.into_iter())
        .collect();

    if illegal_combined.any(|x| x == *tile) {
        *tile = rand::thread_rng().gen_range(0..9);
        check_legality(tile, tile_pos, board);
        println!("legal didn't pass")
    } else {
        println!("legal passed")
    }
    *tile
}
