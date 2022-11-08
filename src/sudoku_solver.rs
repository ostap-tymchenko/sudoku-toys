use comfy_table::{modifiers::UTF8_ROUND_CORNERS, presets::UTF8_FULL, *};
use rand::prelude::*;
use std::collections::HashSet;

pub fn sudoku_solver_main() {
    println!("sudoku_solver.rs init");

    let board = [
        [0, 0, 0, 0, 0, 9, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 4],
        [0, 0, 7, 6, 4, 8, 0, 0, 2],
        [0, 0, 2, 0, 5, 0, 8, 0, 0],
        [0, 0, 1, 8, 0, 0, 0, 2, 3],
        [7, 0, 4, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 3, 0, 0, 0, 4, 0],
        [0, 0, 0, 0, 6, 0, 5, 0, 0],
        [0, 2, 3, 0, 1, 0, 0, 0, 0],
    ];
}

fn post_board(board: [[u8; 9]; 9]) {
    let mut post = Table::new();
    for i in 0..9 {
        post.add_row(board[i]);
    }
    post.load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS);

    println!("{post}");
}

fn solve_board_brute_force_1(mut board: [[u8; 9]; 9]) -> [[u8; 9]; 9] {
    for rows in &mut board {
        for mut tile in rows {
            *tile = rand::thread_rng().gen_range(1..=9);
            check_legality(tile);
        }
    }
    board
}

fn check_legality(tiles: &mut u8) -> u8 {
    
    let illegal_row = [5, 4, 4];
    let illegal_box = [1, 1, 1];

    let mut illegal_combined  = illegal_row.into_iter().chain(illegal_box.into_iter());

    if illegal_combined.any(|x| x == *tiles) {
        *tiles = rand::thread_rng().gen_range(0..9);
        check_legality(tiles);
        println!("legal didn't pass")
    } else {
        println!("legal passed")
    }
    //return
    *tiles
}
