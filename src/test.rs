use std::collections::HashSet;
// comfy tables for displaying, see post_board function
use comfy_table::{modifiers::UTF8_ROUND_CORNERS, presets::UTF8_FULL, *};

pub fn test_main() {
    let board = [[1..9]];

    // println!("{}", board.as_slice(s![1..4, 1..4]).iter().copied().collect::<array1<_>>());
    post_board(board);
}

fn post_board(board: [[i32; 9]; 9]) {
    let mut post = Table::new();
    for i in 1..9 {
        post.add_row(board[i]);
    }
    post.load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS);

    println!("{post}");
}

