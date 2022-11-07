use comfy_table::{*, presets::UTF8_FULL, modifiers::UTF8_ROUND_CORNERS};


pub fn comfy_table() {

    let board: [[u8; 9];9] = [
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

    let mut b_table = Table::new();
    for i in 0..9 {
        b_table
        .add_row(board[i]);
    }

    b_table
    .load_preset(UTF8_FULL)
    .apply_modifier(UTF8_ROUND_CORNERS);

    println!("{b_table}");
}