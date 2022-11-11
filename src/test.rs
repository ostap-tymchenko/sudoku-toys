use rand::Rng;

pub fn main () {  
    let board = [[0; 9]; 9];

    pass (&mut 0, 13, board);
}

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

fn pass (tile: &mut i32, tile_pos:i32, board: [[i32; 9]; 9]) {

    let tile_pos_f32 =f32::from(tile_pos as i32);
    dbg!(tile_pos);

    let row: [i32; 9] = board[[tile_pos_f32 / 9.0].iter()];
    dbg!(row);

    let col: [i32; 9] = board[[tile_pos_f32%9.0].iter().collect()];
    dbg!(col);

    let x_cord = row.iter().map(|x| x/3);
    let y_cord = col.iter().map(|x| x/3);

    // let xy_box = match (x_cord, y_cord) {
    //     (0, 0) => BoxRange::TopLeft,
    //     (0, 1) => BoxRange::TopMiddle,
    //     (0, 2) => BoxRange::TopRight,
    //     (1, 0) => BoxRange::MiddleLeft,
    //     (1, 1) => BoxRange::MiddleMiddle,
    //     (1, 2) => BoxRange::MiddleRight,
    //     (2, 0) => BoxRange::BottomLeft,
    //     (2, 1) => BoxRange::BottomMiddle,
    //     (2, 2) => BoxRange::BottomRight,
    //     _ => panic!("bad math"),
    // };
}

// fn solve_board_brute_force(mut board: [[i32; 9]; 9]) -> [[i32; 9]; 9] {
//     let mut tile_pos:i32 = 0;
//     for rows in &board {
//         for mut tile in rows {
//             tile_pos += 1;
//             *tile = rand::thread_rng().gen_range(1..10);
//             pass(&mut tile, tile_pos, board);
//         }
//     }
//     // post_board(board);
//     board
// }