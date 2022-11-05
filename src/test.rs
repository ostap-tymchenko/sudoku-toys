use tabled::{object::Rows, Table, Tabled};

pub fn main() {
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
        for tiles in board {
            let row_to_str = rows.to_string();
        }
    }

    #[derive(Tabled)]
    struct RowDisplay {
        row: String,
    }

    impl RowDisplay {
        fn new(row: &str) -> Self {
            Self
        }
    }

    let rows = vec![
        RowDisplay::new("snip"),
        // RowDisplay::new(),
        // RowDisplay::new(),
    ];

    let table = Table::new(rows).to_string();

    println!("{}", table);
}
