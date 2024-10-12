mod sudoku;
// mod game as sudoku;

fn main() {
    let sudoku_game: sudoku::SudokuGame = sudoku::get_next_game();
    println!("{sudoku_game:#?}");

    let mut sudoku_puzzle: sudoku::SudokuPuzzle = sudoku::create_puzzle(&sudoku_game);
    sudoku_puzzle.display();

    let solved: bool = sudoku::solve_puzzle(&mut sudoku_puzzle);
    sudoku_puzzle.display();
    println!("game has been {}.", {
        if solved {
            "solved"
        } else {
            "not solved"
        }
    });
}
