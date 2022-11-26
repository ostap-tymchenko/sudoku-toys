// comfy tables for displaying, see post_board function
use comfy_table::{modifiers::UTF8_ROUND_CORNERS, presets::UTF8_FULL, *};
// random for generating a random number for a tile
use rand::prelude::*;
use std::collections::HashSet;
use ndarray::prelude::*;

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

    println!("{}", board.as_slice(s![1..4, 1..4]).iter().copied().collect::<array1<_>>());

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

// this is the initial that takes a board and asigns random valus to non zero tiles
// the value is then checked for validity using check_legality

fn solve_board_brute_force(mut board: [[i32; 9]; 9]) -> [[i32; 9]; 9] {
    let mut tile_pos:i32 = 0;
    for row in board {
        for mut tile in row {
            tile_pos += 1;
            // println!("old ={tile}");
            if tile == 0 {
                let mut tile = rand::thread_rng().gen_range(1..10);
                // println!("new ={tile}");
                check_legality(&mut tile, tile_pos, board);
            } else {/*print!("skipped tile");*/}
        }
    }
    post_board(board);
    board
}
    
fn check_legality(tile: &mut i32, tile_pos:i32, board: [[i32; 9]; 9]) -> i32 {
    let row = board[(tile_pos as f64 / 9.0).floor() as usize ];
    let col = board[(tile_pos as f64 % 9.0) as usize ];

    let possibilities = HashSet::from([1..10]);

    // 1..4 => 1,
    // 4..7 => 2,
    // 7..10 => 3,

    let box = match (row, col) {
        (1..4, 1..4) => (1, 4)
        (1..4, 4..7) => (4, 4)
        (1..4, 7..10) => (7, 4)
        (4..7, 1..4) => (1, 7)
        (4..7, 4..7) => (4, 7)
        (4..7, 7..10) => (7, 7)
        (7..10, 1..4) => (1, 10)
        (7..10, 4..7) => (4, 10)
        (7..10, 7..10) => (7, 10)
    };

    let cur_box = board.slice(s![box.0, box.1]);
    // *tile
    4
}
