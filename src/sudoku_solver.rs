#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]

use rand::prelude::*;
use tabled::{Table, Tabled};

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

    for rows in board {
        println!("{:?}", rows);
        for tiles in rows {
            let row_to_str = String::new(&tiles.to_string());
            println!("{:?}", row_to_str);
        }
    }
}

fn solve_board_brute_force(mut board: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    for rows in &mut board {
        for mut tiles in rows {
            *tiles = rand::thread_rng().gen_range(0..9);
            check_legality(tiles);
        }
    }
    board
}

fn check_legality(tiles: &mut u8) -> u8 {
    let illegal_values_1 = vec![5];
    let illegal_row = vec![5];

    if illegal_values_1.contains(&tiles) {
        *tiles = rand::thread_rng().gen_range(0..9);
        check_legality(tiles);
        println!("legal didn't pass")
    } else {
        println!("legal passed")
    }
    //return
    *tiles
}
