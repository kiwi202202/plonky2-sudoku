#[derive(Clone, Copy, Debug)]
pub struct Cell {
    pub value: u8,
    pub editable: bool,
}

pub struct Sudoku {
    pub grid: [[Cell; 9]; 9],
}

impl Sudoku {
    pub fn new() -> Self {
        let empty_cell = Cell {
            value: 0,
            editable: true,
        };
        Sudoku {
            grid: [[empty_cell; 9]; 9],
        }
    }

    pub fn from(puzzle: [[u8; 9]; 9]) -> Self {
        let mut sudoku = Sudoku::new();
        for (i, row) in puzzle.iter().enumerate() {
            for (j, &value) in row.iter().enumerate() {
                sudoku.grid[i][j] = Cell {
                    value,
                    editable: value == 0,
                };
            }
        }
        sudoku
    }

    pub fn set_cell(&mut self, row: usize, col: usize, value: u8) {
        if self.grid[row][col].editable {
            self.grid[row][col].value = value;
        }
    }
}

impl Sudoku {
    pub fn is_valid(&self) -> bool {
        self.rows_valid() && self.cols_valid() && self.squares_valid()
    }

    fn rows_valid(&self) -> bool {
        self.grid.iter().all(|row| {
            let values: Vec<u8> = row.iter().map(|cell| cell.value).collect();
            self.group_valid(&values)
        })
    }

    fn cols_valid(&self) -> bool {
        (0..9).all(|col| {
            let col_vals: Vec<u8> = self.grid.iter().map(|row| row[col].value).collect();
            self.group_valid(&col_vals)
        })
    }

    fn squares_valid(&self) -> bool {
        (0..3).all(|r| {
            (0..3).all(|c| {
                let square_vals: Vec<u8> = self
                    .grid
                    .iter()
                    .skip(r * 3)
                    .take(3)
                    .flat_map(|row| row.iter().skip(c * 3).take(3))
                    .map(|cell| cell.value)
                    .collect();
                self.group_valid(&square_vals)
            })
        })
    }

    // fn group_valid(&self, values: &[u8]) -> bool {
    //     let mut seen = [false; 9];
    //     for &value in values {
    //         if value != 0 {
    //             if seen[value as usize - 1] {
    //                 return false;
    //             }
    //             seen[value as usize - 1] = true;
    //         }
    //     }
    //     true
    // }

    fn group_valid(&self, values: &[u8]) -> bool {
        let mut non_zero_values: Vec<u8> = values
            .iter()
            .filter(|&&value| value != 0)
            .cloned()
            .collect();

        non_zero_values.sort();

        non_zero_values == vec![1, 2, 3, 4, 5, 6, 7, 8, 9]
    }
}
