mod model;
mod ui;

use model::Sudoku;
use ui::run_tui;

fn main() {
    let mut sudoku = Sudoku::from([
        [0, 0, 7, 4, 8, 6, 9, 2, 5],
        [5, 6, 8, 2, 1, 9, 3, 4, 7],
        [2, 4, 9, 7, 3, 5, 6, 8, 1],
        [8, 5, 1, 3, 2, 7, 4, 6, 9],
        [7, 9, 2, 8, 6, 4, 1, 5, 3],
        [6, 3, 4, 9, 5, 1, 2, 7, 8],
        [1, 2, 3, 6, 7, 8, 5, 9, 4],
        [4, 7, 6, 5, 9, 3, 8, 1, 2],
        [9, 8, 5, 1, 4, 2, 7, 3, 6],
    ]);
    if let Err(err) = run_tui(&mut sudoku) {
        println!("Error running TUI: {}", err);
    }
}
