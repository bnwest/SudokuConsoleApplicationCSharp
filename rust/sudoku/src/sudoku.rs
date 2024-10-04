// Lessons learned:
//
// 1. String processing is a no fly zone.  strings are are an array of potentially
// variable length utf-8 characters.  Indexing into a string does nto make sense.
// My workaround was to create an array of characters, where indexing works.
//
// 2. Indices must use type "usize".  Rust is very unforgiving.
//
// 3. types are not required at variable declaration.  rust compiler can decide
// from context what the type is, making the type implicit.  for loop variables
// get context from fro statement and may enough context for integer types.
//
// 4. variable must be initialized at declaration.  might be required since types are not.
//
// 5. rust compiler warns on constant variable not being all CAPs,
// expressions have "unneeded" parens, etc aka stuff that does not matter.
//
// 6. arrays are declared ass backwards ... aka inside out.
// [[uside; 3]; 9] => a 9 row x 3 column array.
//

#[derive(Debug)]  // adding so prety print will work ... {:#?} for pretty-print
pub struct SudokuGame {
    // private data member(s)
    db_game: String,
}

impl SudokuGame {
    fn create() -> SudokuGame {
        // randomly create a game
        return SudokuGame {
            // db_game is a 81 character string, with the solved numbers present.
            db_game: String::from(
                // hidden triples
                "4....8.....753...8.9..6.41353....2.7.........7.6....81954.1..3.3...751.....9....5"
                // 4..  ..8  ...
                // ..7  53.  ..8
                // .9.  .6.  413
                //
                // 53.  ...  2.7
                // ...  ...  ...
                // 7.6  ...  .81
                //
                // 954  .1.  .3.
                // 3..  .75  1..
                // ...  9..  ..5

                // type 1 and 2 locked candidate in row and column
                // "563700000002000947040100000030050209020000080409010050000004010254000600000006495"
            ),
        }
    }
}


#[derive(Clone, Copy, Debug)]  // needed Copy to create 2D array.  needed Clone for Copy.
pub struct SudokuPuzzleCell {
    possibles: [bool; 9],
    found: usize,
}

impl SudokuPuzzleCell {
    const NOT_FOUND: usize = 0xFFFFFFFF;

    fn new() -> SudokuPuzzleCell {
        // create an empty cell
        return SudokuPuzzleCell {
            possibles:  [true; 9],
            found: SudokuPuzzleCell::NOT_FOUND,
        }
    }

    fn initialize(&mut self) {
        // create an empty cell
        self.possibles = [true; 9];
        self.found = SudokuPuzzleCell::NOT_FOUND;
    }

    fn set_possible(&mut self, i: usize, value: bool) {
        self.possibles[i] = value;
    }

    fn is_possible(&self, i: usize) -> bool {
        return self.possibles[i];
    }

    fn is_solved(&self) -> bool {
        let solved: bool = self.found != SudokuPuzzleCell::NOT_FOUND;
        return solved;
    }

    fn set_exclusive_possibles(&mut self, possibles: &[usize; 9]) {
        for i in 0..9 {
            self.possibles[i] = false;
        }
        for i in 0..9 {
            let value: usize = possibles[i];
            if value != SudokuPuzzleCell::NOT_FOUND {
                self.possibles[value] = true;
            }
        }
    }
}


#[derive(Copy, Clone)]
struct CellCoordinate {
    row: usize,
    column: usize,
}


// #[derive(Debug, Clone)]
#[derive(Debug)]
pub struct SudokuPuzzle {
    // rust only supports single dimensional arrays 
    // => one needs build their own multi-dimensional array
    cells: [[SudokuPuzzleCell; 9]; 9],
}

impl SudokuPuzzle {
    fn initialize(&mut self) {
        for row in 0..9 {
            for column in 0..9 {
                self.cells[row][column].initialize();
            }
        }
    }

    fn create(game: &SudokuGame) -> SudokuPuzzle {
        let mut puzzle: SudokuPuzzle = SudokuPuzzle {
            cells: [[SudokuPuzzleCell::new(); 9]; 9],
        };
        puzzle.initialize();

        // cleanup / normalize the inout game strings
        // some have "+" at the en of every row (given 9 rows)
        // some use "0" versus "." to indicate unknown values

        let input_string_has_plus_signs: bool = game.db_game.chars().nth(9).unwrap() == '+';
        let cell_increment: usize = {if input_string_has_plus_signs {10} else {9}};

        let mut normalized_db_game: String = String::from("");

        for i in (0..81).step_by(cell_increment) {
            normalized_db_game.push_str(&game.db_game[i..i+9]);
        }

        // lesson learned: rust does not support string processing it is hard.
        let mut normalized_db_game_char_array: [char; 81] = ['.'; 81];
        for (i, c) in normalized_db_game.chars().enumerate() {
            if '1' <= c && c <= '9' {
                normalized_db_game_char_array[i] = c;
            }
        }
        // println!("create: normalized_db_game {normalized_db_game}");

        // convert normalized input game string into a Sudoku puzzle

        let mut input_index: usize = 0;

        for row in 0..9 {
            let input_row = &normalized_db_game_char_array[input_index..input_index+9];
            for column in 0..9 {
                let cell: char = input_row[column];
                // println!("create: row {row} column {column} cell {cell}.");
                if '1' <= cell && cell <= '9' {
                    // convert '1'..'9' into 0..8, for internal futzing.
                    let cell_digit: usize = cell as usize - '1' as usize;
                    for i in 0..9 {
                        puzzle.cells[row][column].set_possible(i, false);
                    }
                    // know that this cell has a solution,
                    // but will let find_naked_singles() discover it and clean up neighbors
                    puzzle.cells[row][column].set_possible(cell_digit, true);
                }
            }
            input_index += 9;
        }

        return puzzle;
    }

    pub fn display(&self) {
        // a row containing 9 cells:
        //      "+-------+ +-------+ +-------+  +-------+ +-------+ +-------+  +-------+ +-------+ +-------+\n"
        //      "| 1 2 3 | | 1 2 3 | | 1 2 3 |  | 1 2 3 | | 1 2 3 | | 1 2 3 |  | 1 2 3 | | 1 2 3 | | 1 2 3 |\n"
        //      "| 4 5 6 | | 4 5 6 | | 4 5 6 |  | 4 5 6 | | 4 5 6 | | 4 5 6 |  | 4 5 6 | | 4 5 6 | | 4 5 6 |\n"
        //      "| 7 8 9 | | 7 8 9 | | 7 8 9 |  | 7 8 9 | | 7 8 9 | | 7 8 9 |  | 7 8 9 | | 7 8 9 | | 7 8 9 |\n"
        //      "+-------+ +-------+ +-------+  +-------+ +-------+ +-------+  +-------+ +-------+ +-------+\n"

        for i in 0..9 {
            println!("+-------+ +-------+ +-------+  +-------+ +-------+ +-------+  +-------+ +-------+ +-------+");
            for k in (0..9).step_by(3) {
                for j in 0..9 {
                    let cell: SudokuPuzzleCell = self.cells[i][j];

                    let kk: usize = usize::try_from(k).unwrap();

                    let char1: char = {if cell.is_possible(kk+0) {char::from_digit(k+1, 10).expect("digit is digit")} else{' '}};
                    let char2: char = {if cell.is_possible(kk+1) {char::from_digit(k+2, 10).expect("digit is digit")} else{' '}};
                    let char3: char = {if cell.is_possible(kk+2) {char::from_digit(k+3, 10).expect("digit is digit")} else{' '}};

                    print!("| {char1} {char2} {char3} | ");
                    if (j + 1) % 3 == 0 {
                        print!(" ");
                    }
                }
                println!();
            }
            println!("+-------+ +-------+ +-------+  +-------+ +-------+ +-------+  +-------+ +-------+ +-------+");
            if (i + 1) % 3 == 0 {
                println!();
            }
        }
    }

    fn find_naked_singles(&mut self) -> bool {
        let mut found_singles: bool = false;
        let mut singles: [[usize; 9]; 9] = [[SudokuPuzzleCell::NOT_FOUND; 9]; 9];

        for i in 0..9 {
            for j in 0..9 {
                let cell: SudokuPuzzleCell = self.cells[i][j];
                if !cell.is_solved() {
                    let mut num_possibilities: u8 = 0;
                    let mut last_found: usize = SudokuPuzzleCell::NOT_FOUND;
                    for k in 0..9 {
                        if cell.is_possible(k) {
                            last_found = k;
                            num_possibilities += 1;
                            if num_possibilities > 1 {
                                break;
                            }
                        }
                    }
                    let found_single: bool = num_possibilities == 1;
                    if found_single {
                       found_singles = true;
                       singles[i][j] = last_found;
                    }
                    else {
                        singles[i][j] = SudokuPuzzleCell::NOT_FOUND;
                    }
                }
                else {
                    singles[i][j] = SudokuPuzzleCell::NOT_FOUND;
                }
            }
        }

        if found_singles {
            for i in 0..9 {
                for j in 0..9 {
                    let found_single: bool = singles[i][j] != SudokuPuzzleCell::NOT_FOUND;
                    if found_single {
                        let k = singles[i][j];
                        {
                            let ii = i + 1;
                            let jj = j + 1;
                            let kk = k + 1;
                            println!("found naked single {kk} at cell({ii}, {jj}).");
                        }
                        // mark cell as a single
                        self.solve_cell(i, j, k);
                    }
                }
            }
        }

        return found_singles;
    }

    fn find_horizontal_exclusions(
         &mut self,
         assist: fn(puzzle: &mut SudokuPuzzle, cells: &[CellCoordinate; 9], description: String) -> u32
    ) -> bool {
        let mut num_changes: u32 = 0;
        let mut cells: [CellCoordinate; 9] =  [
            CellCoordinate{row: SudokuPuzzleCell::NOT_FOUND, column: SudokuPuzzleCell::NOT_FOUND}; 9
        ];

        for row in 0..9 {
            for column in 0..9 {
                cells[column].row = row;
                cells[column].column = column;
            }
            // let puzzle: &mut SudokuPuzzle = self;
            num_changes += assist(self, &cells, format!("for row {}", row+1));
        }

        return num_changes > 0;
    }

    fn find_vertical_exclusions(
         &mut self,
         assist: fn(puzzle: &mut SudokuPuzzle, cells: &[CellCoordinate; 9], description: String) -> u32
    ) -> bool {
        let mut num_changes: u32 = 0;
        let mut cells: [CellCoordinate; 9] =  [
            CellCoordinate{row: SudokuPuzzleCell::NOT_FOUND, column: SudokuPuzzleCell::NOT_FOUND}; 9
        ];

        for column in 0..9 {
            for row in 0..9 {
                cells[row].row = row;
                cells[row].column = column;
            }
            num_changes += assist(self, &cells, format!("for column {}", column+1));
        }

        return num_changes > 0;
    }

    fn find_grid_exclusions(
         &mut self,
         assist: fn(puzzle: &mut SudokuPuzzle, cells: &[CellCoordinate; 9], description: String) -> u32
    ) -> bool {
        let mut num_changes: u32 = 0;
        let mut cells: [CellCoordinate; 9] =  [
            CellCoordinate{row: SudokuPuzzleCell::NOT_FOUND, column: SudokuPuzzleCell::NOT_FOUND}; 9
        ];

        for gridi in 0..3 {
            for gridj in 0..3 {
                let starti: usize = gridi * 3;
                let startj: usize = gridj * 3;

                let mut grid_index: usize = 0;
                for row in starti..starti+3 {
                    for column in startj..startj+3 {
                        cells[grid_index].row = row;
                        cells[grid_index].column = column;
                        grid_index += 1;
                    }
                }
                num_changes += assist(self, &cells, format!("for grid({}, {})", gridi+1, gridj+1));
            }
        }

        return num_changes > 0;
    }

    fn find_exclusions(
        &mut self,
        assist: fn(puzzle: &mut SudokuPuzzle, cells: &[CellCoordinate; 9], description: String) -> u32
    ) -> bool {
        let mut found_exclusions: bool = false;

        found_exclusions |= self.find_horizontal_exclusions(assist);
        found_exclusions |= self.find_vertical_exclusions(assist);
        found_exclusions |= self.find_grid_exclusions(assist);

        return found_exclusions;
    }

    fn find_hidden_singles(&mut self) -> bool {
        return self.find_exclusions(SudokuPuzzle::find_hidden_singles_assist);
    }

    fn find_hidden_singles_assist(
        puzzle: &mut SudokuPuzzle, cells: &[CellCoordinate; 9], description: String
    ) -> u32 {
        let mut num_changes = 0;

        for k in 0..9 {
            let mut num_possibilities = 0;
            let mut last_found: usize = SudokuPuzzleCell::NOT_FOUND;
            for c in 0..9 {
                let row: usize = cells[c].row;
                let column: usize = cells[c].column;
                if !puzzle.cells[row][column].is_solved() {
                    if puzzle.cells[row][column].is_possible(k) {
                        // println!(
                        //     "find_hidden_singles_assist: row({}) column({}) possible({})? {}, desc({}).",
                        //     row+1, column+1, k+1, puzzle.cells[row][column].is_possible(k), description
                        // );
                        num_possibilities += 1;
                        last_found = c;
                    }
                }
            }

            let got_hidden_single: bool = num_possibilities == 1;
            if got_hidden_single {
                let row: usize = cells[last_found].row;
                let column: usize = cells[last_found].column;
                println!(
                    "found hidden single {} in cell({}, {}) {}.",
                    k+1,
                    row+1,
                    column+1,
                    description
                );
                puzzle.solve_cell(row, column, k);
                num_changes += 1;
            }
        }

        return num_changes;
    }


    fn find_locked_candidate_horizontal(&mut self) -> bool {
        let mut num_changes: u32 = 0;
        let mut candidate: [[[bool; 9]; 3]; 9] = [[[false; 9]; 3]; 9];

        for row in 0..9 {
           for k in 0..9 {
               candidate[row][0][k] = false;
               candidate[row][1][k] = false;
               candidate[row][2][k] = false;
           }
           for k in 0..9 {
              for column in 0..9 {
                  if !self.cells[row][column].is_solved() {
                      let gridj: usize = column / 3;
                      candidate[row][gridj][k] |= self.cells[row][column].is_possible(k);
                  }
              }
           }
        }

        //
        // Locked Candidate Type 1
        // if single is in the grid's row/column but is not in the grid's other row/column,
        // then we can exclude the single from the rest of the row/column.
        // IOW, a candidate for a grid is only in a single row/column and thus the grid
        // constraint forces the candidate out of the remainder of the row/column.
        //

        for gridi in 0..3 {
            for gridj in 0..3 {
                let starti: usize = gridi * 3;
                let startj: usize = gridj * 3;
                for k in 0..9 {
                    let mut found_locked_candidate: bool = false;
                    let mut locked_row: usize = SudokuPuzzleCell::NOT_FOUND;

                    if candidate[starti][gridj][k]
                        && !candidate[starti+1][gridj][k]
                        && !candidate[starti+2][gridj][k] {
                        found_locked_candidate = true;
                        locked_row = starti;
                    }
                    if !candidate[starti][gridj][k]
                        && candidate[starti+1][gridj][k]
                        && !candidate[starti+2][gridj][k] {
                        found_locked_candidate = true;
                        locked_row = starti + 1;
                    }
                    if !candidate[starti][gridj][k]
                        && !candidate[starti+1][gridj][k]
                        && candidate[starti+2][gridj][k] {
                        found_locked_candidate = true;
                        locked_row = starti + 2;
                    }

                    if found_locked_candidate {
                        println!(
                            "found type 1 locked candidate {0} in row cells({1},{2}..{3}), exclude the rest of the row",
                            k+1, locked_row+1, startj+1, startj+3
                        );
                        for column in 0..9 {
                            let this_grid_column: usize = column / 3;
                            let grid_needing_exclusion: bool = this_grid_column != gridj;
                            if grid_needing_exclusion {
                                if self.cells[locked_row][column].is_possible(k) {
                                    num_changes += 1;
                                    self.cells[locked_row][column].set_possible(k, false);
                                    println!(
                                        "    exclude {} from cell({}, {}).",
                                        k+1, locked_row+1, column+1
                                    );
                                }
                            }
                        }
                    }
                }
            }
        }

        //
        // Locked Candidate Type 2
        // if the single is in the grid's row/column but not in the rest of the row/column,
        // then we can restrict the single from the rest of the grid.
        // IOW, a candidate for a row/column is only in a single grid and the the row/column
        // constraint forces the candidate out of the remainder of the grid.
        //

        for row in 0..9 {
            for k in 0..9 {
                let mut found_locked_candidate: bool = false;
                let mut startj: usize = SudokuPuzzleCell::NOT_FOUND;

                if candidate[row][0][k]
                    && !candidate[row][1][k]
                    && !candidate[row][2][k] {
                    found_locked_candidate = true;
                    startj = 0;
                }
                if !candidate[row][0][k]
                    && candidate[row][1][k]
                    && !candidate[row][2][k] {
                    found_locked_candidate = true;
                    startj = 3;
                }
                if !candidate[row][0][k]
                    && !candidate[row][1][k]
                    && candidate[row][2][k] {
                    found_locked_candidate = true;
                    startj = 6;
                }

                if found_locked_candidate {
                    let gridj: usize = startj / 3;
                    let starti: usize = row / 3 * 3;
                    let found_possibilities_to_remove: bool = (
                        starti != row && candidate[starti][gridj][k]
                        || starti+1 != row && candidate[starti+1][gridj][k]
                        || starti+2 != row && candidate[starti+2][gridj][k]
                    );
                    if found_possibilities_to_remove {
                        println!(
                            "found type 2 locked candidate {} in row cells({},{}..{}), exclude the rest of the grid",
                            k+1, row+1, startj+1, startj+3
                        );
                        for i in starti..starti+3 {
                            let row_needing_exclusion: bool = i != row;
                            if row_needing_exclusion {
                                for j in startj..startj+3 {
                                    if self.cells[i][j].is_possible(k) {
                                        num_changes += 1;
                                        self.cells[i][j].set_possible(k, false);
                                        println!(
                                            "    exclude {} from cell({}, {}).",
                                            k+1, i+1, j+1
                                        );
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        return num_changes > 0;
    }

    fn find_locked_candidate_vertical(&mut self) -> bool {
        let mut num_changes: u32 = 0;
        let mut candidate: [[[bool; 9]; 3]; 9] = [[[false; 9]; 3]; 9];

        for column in 0..9 {
            for k in 0..9 {
                candidate[column][0][k] = false;
                candidate[column][1][k] = false;
                candidate[column][2][k] = false;
            }
            for k in 0..9 {
                for row in 0..9 {
                    if !self.cells[row][column].is_solved() {
                        let  gridi: usize = row / 3;
                        candidate[column][gridi][k] |= self.cells[row][column].is_possible(k);
                    }
                }
            }
        }

        //
        // Locked Candidate Type 1
        // if single is in the grid's row/column but is not in the grid's other row/column,
        // then we can exclude the single from the rest of the row/column
        //

        for gridi in 0..3 {
            for gridj in 0..3 {
                let starti: usize = gridi * 3;
                let startj: usize = gridj * 3;
                for k in 0..9 {
                    let mut found_locked_candidate: bool = false;
                    let mut locked_column: usize = SudokuPuzzleCell::NOT_FOUND;
                    // if ( Candidate[startj, gridi, k] && !Candidate[startj + 1, gridi, k] && !Candidate[startj + 2, gridi, k] )
                    if candidate[startj][gridi][k]
                        && !candidate[startj+1][gridi][k]
                        && !candidate[startj+2][gridi][k] {
                        found_locked_candidate = true;
                        locked_column = startj;
                    }
                    if !candidate[startj][gridi][k]
                        && candidate[startj+1][gridi][k]
                        && !candidate[startj+2][gridi][k] {
                        found_locked_candidate = true;
                        locked_column = startj + 1;
                    }
                    if !candidate[startj][gridi][k]
                        && !candidate[startj+1][gridi][k]
                        && candidate[startj+2][gridi][k] {
                        found_locked_candidate = true;
                        locked_column = startj + 2;
                    }
                    if found_locked_candidate {
                        // row includes
                        //     candidate[locked_row][0][k]
                        //     candidate[locked_row][1][k]
                        //     candidate[locked_row][2][k]
                        let found_possibilities_to_remove: bool = (
                            gridi != 0 && candidate[locked_column][0][k]
                            || gridi != 1 && candidate[locked_column][1][k]
                            || gridi != 2 && candidate[locked_column][2][k]
                        );
                        if found_possibilities_to_remove {
                            println!(
                                "found type 1 locked candidate {} in column cells({}..{},{}), exclude the rest of the column",
                                k+1, starti+1, starti+3, locked_column+1
                            );
                            for row in 0..9 {
                                let this_grid_row: usize = row / 3;
                                let grid_needing_exclusion: bool = this_grid_row != gridi;
                                if grid_needing_exclusion {
                                    if self.cells[row][locked_column].is_possible(k) {
                                        num_changes += 1;
                                        self.cells[row][locked_column].set_possible(k, false);
                                        println!(
                                            "    exclude {} from cell({}, {}).",
                                            k+1, row+1, locked_column+1
                                        );
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        //
        // Locked Candidate Type 2
        // if the single is in the grid's row/column but not in the rest of the row/column,
        // then we can restrict the single from the rest of the grid
        //

        for column in 0..9 {
            for k in 0..9 {
                let mut found_locked_candidate = false;
                let mut starti: usize = SudokuPuzzleCell::NOT_FOUND;
                // Candidate[column, 0, k] && !Candidate[column, 1, k] && !Candidate[column, 2, k] )
                if candidate[column][0][k]
                    && !candidate[column][1][k]
                    && !candidate[column][2][k] {
                    found_locked_candidate = true;
                    starti = 0;
                }
                if !candidate[column][0][k]
                    && candidate[column][1][k]
                    && !candidate[column][2][k] {
                    found_locked_candidate = true;
                    starti = 3;
                }
                if !candidate[column][0][k]
                    && !candidate[column][1][k]
                    && candidate[column][2][k] {
                    found_locked_candidate = true;
                    starti = 6;
                }
                if found_locked_candidate {
                    let gridi = starti / 3;
                    let startj: usize = column / 3 * 3;
                    // grid includes
                    //     candidate[starti][gridj][k]
                    //     candidate[starti+1][gridj][k]
                    //     candidate[starti+2][gridj][k]
                    let found_possibilities_to_remove: bool = (
                        (startj != column && candidate[startj][gridi][k])
                        || (startj + 1 != column && candidate[startj + 1][gridi][k])
                        || (startj + 2 != column && candidate[startj + 2][gridi][k])
                    );
                    if found_possibilities_to_remove {
                        println!(
                            "found type 2 locked candidated {} in column cells({}..{},{}), exclude the rest of the grid",
                            k+1, starti+1, starti+3, column+1
                        );
                        for j in startj..startj+3 {
                            let column_needing_exclusion: bool = j != column;
                            if column_needing_exclusion {
                                for i in starti..starti+3 {
                                    if self.cells[i][j].is_possible(k) {
                                        num_changes += 1;
                                        self.cells[i][j].set_possible(k, false);
                                        println!(
                                            "    excluding {} in cell({}, {}).",
                                            k+1, i+1, j+1
                                        );
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        return num_changes > 0;
    }

    //
    // Locked Candidate Type 1
    // if single is in the grid's row/column but is not in the grid's other row/column,
    // then we can exclude the single from the rest of the row/column.
    // IOW, a candidate for a grid is only in a single row/column and thus the grid
    // constraint forces the candidate out of the remainder of the row/column.
    //
    // Locked Candidate Type 2
    // if the single is in the grid's row/column but not in the rest of the row/column,
    // then we can restrict the single from the rest of the grid.
    // IOW, a candidate for a row/column is only in a single grid and the the row/column
    // constraint forces the candidate out of the remainder of the grid.
    //

    fn find_locked_candidate(&mut self) -> bool {
        let mut did_something: bool = false;
        did_something |= self.find_locked_candidate_horizontal();
        did_something |= self.find_locked_candidate_vertical();
        return did_something;
    }

    fn find_naked_pairs_assist(
        puzzle: &mut SudokuPuzzle, cells: &[CellCoordinate; 9], description: String
    ) -> u32 {
        let mut num_changes: u32 = 0;

        let mut candidate_pairs: [[usize; 3]; 9] = [[SudokuPuzzleCell::NOT_FOUND; 3]; 9];  // 9 row x 3 column array
        let mut num_candidate_pairs: usize = 0;

        // File "iter()" and iter_mut" under ... WTF.
        for (c, cell) in cells.iter().enumerate() {
            let row: usize = cell.row;
            let column: usize = cell.column;

            let mut p1: usize = SudokuPuzzleCell::NOT_FOUND;
            let mut p2: usize = SudokuPuzzleCell::NOT_FOUND;
            let mut num_possibilities: u32 = 0;

            for k in 0..9 {
                if !puzzle.cells[row][column].is_solved() {
                    if puzzle.cells[row][column].is_possible(k) {
                        num_possibilities += 1;
                        if num_possibilities > 2 {
                            break;
                        }
                        if p1 == SudokuPuzzleCell::NOT_FOUND {
                            p1 = k;
                        }
                        else {
                            p2 = k;
                        }
                    }
                }
            }

            if num_possibilities == 2 {
                // we have found a pair candidate
                candidate_pairs[num_candidate_pairs][0] = p1;
                candidate_pairs[num_candidate_pairs][1] = p2;
                candidate_pairs[num_candidate_pairs][2] = c;
                num_candidate_pairs += 1;
            }
        }

        let mut num_naked_pairs = 0;
        let mut naked_pairs: [[usize; 4]; 9] = [[SudokuPuzzleCell::NOT_FOUND; 4]; 9];  // 9 x 4 array

        if num_candidate_pairs >= 2 {
            for c1 in 0_usize..num_candidate_pairs-1 {
                for c2 in c1+1..num_candidate_pairs {
                    // determine if candidate_pairs[c1] and candidate_pairs[c2]
                    // are candidate pairs.

                    let mut p1: usize = SudokuPuzzleCell::NOT_FOUND;
                    let mut p2: usize = SudokuPuzzleCell::NOT_FOUND;
                    let mut possibles_for_pair: [bool; 9] = [false; 9];

                    p1 = candidate_pairs[c1][0];
                    p2 = candidate_pairs[c1][1];
                    possibles_for_pair[p1] = true;
                    possibles_for_pair[p2] = true;

                    p1 = candidate_pairs[c2][0];
                    p2 = candidate_pairs[c2][1];
                    possibles_for_pair[p1] = true;
                    possibles_for_pair[p2] = true;

                    // let num_possibles_for_pairs: u32 = {
                    //     let mut num: u32 = 0;
                    //     for i in 0..9 {
                    //         if possibles_for_pair[i] {
                    //             num += 1;
                    //         }
                    //     }
                    //     num
                    // };

                    let mut num_possibles_for_pairs: u32 = 0;
                    p1 = SudokuPuzzleCell::NOT_FOUND;
                    p2 = SudokuPuzzleCell::NOT_FOUND;
                    for k in 0..9 {
                        if possibles_for_pair[k] {
                            num_possibles_for_pairs += 1;
                            if p1 == SudokuPuzzleCell::NOT_FOUND {
                                p1 = k;
                            }
                            else {
                                p2 = k;
                            }
                        }
                    }
                    if num_possibles_for_pairs == 2 {
                        // candidate_pairs[c1] and candidate_pairs[c2]
                        // have the same pair: (p1, p2).
                        naked_pairs[num_naked_pairs][0] = p1;
                        naked_pairs[num_naked_pairs][1] = p2;
                        naked_pairs[num_naked_pairs][2] = candidate_pairs[c1][2];  // index into cells[]
                        naked_pairs[num_naked_pairs][3] = candidate_pairs[c2][2];
                        num_naked_pairs += 1;
                    }
                }
            }
        }

        if num_naked_pairs > 0 {
            for t in 0..num_naked_pairs {
                let p1: usize = naked_pairs[t][0];
                let p2: usize = naked_pairs[t][1];
                let c1: usize = naked_pairs[t][2];
                let c2: usize = naked_pairs[t][3];

                let mut num_changes_for_pair: u32 = 0;
                for (c, cell) in cells.iter().enumerate() {
                    let cell_outside_of_pair: bool = (c != c1) && (c != c2);
                    if !cell_outside_of_pair {
                        continue;
                    }

                    let row: usize = cell.row;
                    let column: usize = cell.column;
                    if !puzzle.cells[row][column].is_solved() {
                        if puzzle.cells[row][column].is_possible(p1) {
                            puzzle.cells[row][column].set_possible(p1, false);
                            num_changes_for_pair += 1;
                            num_changes += 1;
                        }
                        if puzzle.cells[row][column].is_possible(p2) {
                            puzzle.cells[row][column].set_possible(p2, false);
                            num_changes_for_pair += 1;
                            num_changes += 1;
                        }
                    }
                }
                if num_changes_for_pair > 0 {
                    println!(
                        "found naked pair ({}, {}) {}",
                        p1+1, p2+1, description
                    );
                }
                else {
                    println!(
                        "xxx found naked pair ({}, {}) {}",
                        p1+1, p2+1, description
                    );
                }
            }
        }

        return num_changes;
    }

    fn find_naked_pairs(&mut self) -> bool {
        return self.find_exclusions(SudokuPuzzle::find_naked_pairs_assist);
    }

    fn solve_cell(&mut self, i: usize, j: usize, k: usize) {
        self.cells[i][j].found = k;

        // other possibilities may need to be removed from cell
        let mut exclusive_possibles : [usize; 9] = [SudokuPuzzleCell::NOT_FOUND; 9];
        exclusive_possibles [0] = k;
        self.cells[i][j].set_exclusive_possibles(&exclusive_possibles);

        self.remove_neighbors(i, j, k);
    }

    fn remove_horizontal_neighbors(&mut self, i: usize, j: usize, k: usize) -> bool {
        let mut found_removal: bool = false;

        for column in 0..9 {
            let is_cell_to_change: bool = column != j;
            if is_cell_to_change {
                // note:
                // let mut cell: SudokuPuzzleCell = self.cells[i][j];
                // creates a copy of the cell => any changes are lost in the ether!!!

                let is_change_needed: bool = self.cells[i][column].is_possible(k);
                if is_change_needed {
                    found_removal = true;
                    self.cells[i][column].set_possible(k, false);
                }
            }
        }

        return found_removal;
    }

    //
    // puzzle[i, j] solved with value k
    // remove vertical neighbors as a possibility for k
    //

    fn remove_vertical_neighbors(&mut self, i: usize, j: usize, k: usize) -> bool {
        let mut found_removal: bool = false;

        for row in 0..9 {
            let is_cell_to_change: bool = row != i;
            if is_cell_to_change {
                let is_change_needed: bool = self.cells[row][j].is_possible(k);
                if is_change_needed {
                    found_removal = true;
                    self.cells[row][j].set_possible(k, false);
                }
            }
        }

        return found_removal;
    }

    //
    // puzzle[i, j] solved with value k
    // remove grid neighbors as a possibility for k
    //

    fn remove_grid_neighbors(&mut self, i: usize, j: usize, k: usize) -> bool {
        let mut found_removal: bool = false;

        let starti: usize = i / 3 * 3;
        let startj: usize = j / 3 * 3;

        for ii in starti..starti+3 {
            for jj in startj..startj+3 {
                let is_cell_to_change: bool = ii != i || jj != j;
                if is_cell_to_change {
                    let is_change_needed: bool = self.cells[ii][jj].is_possible(k);
                    if is_change_needed {
                        found_removal = true;
                        self.cells[ii][jj].set_possible(k, false);
                    }
                }
            }
        }

        return found_removal;
    }

    fn remove_neighbors(&mut self, i: usize, j: usize, k: usize) -> bool {
        let mut found_removal: bool = false;

        found_removal |= self.remove_horizontal_neighbors(i, j, k);
        found_removal |= self.remove_vertical_neighbors(i, j, k);
        found_removal |= self.remove_grid_neighbors(i, j, k);

        return found_removal;
    }

    fn solve(&mut self) -> bool {
        //
        // Solve the puzzle using the easiest to hardest methods.
        // Restart after every change
        //
        // Easy
        // 1. naked singles
        // 2. hidden singles
        // Standard/Modearate
        // 3. locked candidates type 1 and 2
        // 4. naked pairs

        // Hard
        // 5. naked triples
        // 6. naked quads
        // Harder
        // 7. hidden pairs
        // 8. hidden triples
        //----- do not plan to implement the methods below -----
        // 9. hidden quads
        // Expert
        // x-wing, y-wing, swordfish, et al
        //

        loop {
            // Easy

            let did_something: bool = self.find_naked_singles();
            if did_something {
                continue;
            }

            let did_something: bool = self.find_hidden_singles();
            if did_something {
                continue;
            }

            // Standard

            let did_something: bool = self.find_locked_candidate();
            if did_something {
                continue;
            }

            let did_something: bool = self.find_naked_pairs();
            if did_something {
                continue;
            }

            break;
        }

        return self.is_solved();
    }

    pub fn is_solved(&self) -> bool {
        let mut got_unsolved_cell: bool = false;
        for i in 0..9 {
            for j in 0..9 {
                if !self.cells[i][j].is_solved() {
                    got_unsolved_cell = true;
                }
            }
        }
        return !got_unsolved_cell;
    }
}


pub fn get_next_game() -> SudokuGame {
   return SudokuGame::create();
}


pub fn create_puzzle(game: &SudokuGame) -> SudokuPuzzle {
    let mut puzzle: SudokuPuzzle = SudokuPuzzle::create(game);

    // let is_possible = puzzle.cells[0][1].is_possible(3);
    // println!("create_puzzle: before is cell(1, 2, 4) possible? {is_possible}.");

    puzzle.find_naked_singles();
    // let is_possible = puzzle.cells[0][1].is_possible(3);
    // println!("create_puzzle: after is cell(1, 2, 4) possible? {is_possible}.");

    // println!("puzzle: {puzzle:#?}");
    return puzzle;
}


pub fn solve_puzzle(puzzle: &mut SudokuPuzzle) -> bool {
    // freshly created puzzle may have nake singles, so solve them first.
    let solved: bool = puzzle.solve();
    return solved;
}
