using System;


namespace Sudoku.ConsoleApplication
{
    class Program
    {
        static void Main(string[] args)
        {
            Sudoko.SudokuPuzzle puzzle = new Sudoko.SudokuPuzzle();
            puzzle.executeOnConsole();
        }
    }
}
