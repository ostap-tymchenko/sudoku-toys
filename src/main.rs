#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]

mod sudoku_solver;
// mod test;

use rand::prelude::*;

fn main() {
    sudoku_solver::sudoku_solver_main();
    // test::comfy_table();
    // fill_board();
}

