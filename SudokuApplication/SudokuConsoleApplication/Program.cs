using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace Sudoko
{
    class SudokuCell
    {
        public const int NOTFOUND = -1;  // a const object is always static by definition

        private int found;               // field
        public int Found { get; set; }   // property

        public bool IsSolved { get { return (Found != SudokuCell.NOTFOUND); } }

        private bool[] possible = new bool[9];

        public bool isPossible(int i)
        {
            return possible[i];
        }

        public void setPossible(int i, bool value = true)
        {
            possible[i] = value;
        }

        public SudokuCell()
        {
            // allocate sudoku call
        }

        public void initialize()
        {
            // initialize sudoku call

            found = NOTFOUND;

            for (int i = 0; i < possible.GetLength(0); i++)
            {
                setPossible(i, true);
            }
        }
    }

    class SudokuPuzzle
    {
        SudokuCell[,] puzzle = new SudokuCell[9, 9];

        public SudokuPuzzle()
        {
            // allocate puzzle

            for (int i = 0; i < 9; i++)
            {
                for (int j = 0; j < 9; j++)
                {
                    puzzle[i, j] = new SudokuCell();
                }
            }
        }

        private String getInput()
        {
            return "5637....." +
                   "..2...947" +
                   ".4.1....." +
                   ".3..5.2.9" +
                   ".2.....8." +
                   "4.9.1..5." +
                   ".....4.1." +
                   "254...6.." +
                   ".....6495";
            // "563700000002000947040100000030050209020000080409010050000004010254000600000006495"
        }

        public void initialize()
        {
            // initialize puzzle
            for (int i = 0; i < 9; i++)
            {
                for (int j = 0; j < 9; j++)
                {
                    puzzle[i, j].initialize();
                }
            }

            // read input string
            String input = getInput();

            // string contains rows that are either contiguous or separated bya '+'
            int inputIncrement = (input[9] == '+' ? 10 : 9);
            int inputIndex = 0;

            for (int row = 0; row < 9; row++)
            {
                String inputRow = input.Substring(inputIndex, 9);
                for (int column = 0; column < 9; column++)
                {
                    Char cell = inputRow[column];
                    if (Char.IsDigit(cell))
                    {
                        // '1' <= cell <= '9', need to make zero base to internal futzing
                        int cellDigit = Convert.ToInt32(cell - '1');
                        cellDigit = (int)Char.GetNumericValue(cell) - 1;
                        for (int i = 0; i < 9; i++)
                        {
                            puzzle[row, column].setPossible(i, false);
                        }
                        puzzle[row, column].Found = cellDigit;
                        puzzle[row, column].setPossible(cellDigit, true);
                    }
                }
                inputIndex += inputIncrement;
            }
        }

        public void displayPuzzle()
        {
            //      "+-------+ +-------+ +-------+  +-------+ +-------+ +-------+  +-------+ +-------+ +-------+\n"
            //      "| 1 2 3 | | 1 2 3 | | 1 2 3 |  | 1 2 3 | | 1 2 3 | | 1 2 3 |  | 1 2 3 | | 1 2 3 | | 1 2 3 |\n"
            //      "| 4 5 6 | | 4 5 6 | | 4 5 6 |  | 4 5 6 | | 4 5 6 | | 4 5 6 |  | 4 5 6 | | 4 5 6 | | 4 5 6 |\n"
            //      "| 7 8 9 | | 7 8 9 | | 7 8 9 |  | 7 8 9 | | 7 8 9 | | 7 8 9 |  | 7 8 9 | | 7 8 9 | | 7 8 9 |\n"
            //      "+-------+ +-------+ +-------+  +-------+ +-------+ +-------+  +-------+ +-------+ +-------+\n"
            for (int i = 0; i < 9; i++)
            {
                Console.WriteLine("+-------+ +-------+ +-------+  +-------+ +-------+ +-------+  +-------+ +-------+ +-------+");
                for (int k = 0; k < 9; k += 3)
                {
                    for (int j = 0; j < 9; j++)
                    {
                        SudokuCell cell = puzzle[i, j];
                        Char char1 = Convert.ToChar(k + 1);
                        Char char2 = Convert.ToChar(k + 2);
                        Char char3 = Convert.ToChar(k + 3);
                        char1 = Convert.ToString(k + 1)[0];
                        char2 = Convert.ToString(k + 2)[0];
                        char3 = Convert.ToString(k + 3)[0];
                        char1 = char2 = char3;
                        Console.Write("| {0} {1} {2} | ",
                            (puzzle[i, j].isPossible(k) ? Convert.ToString(k + 1)[0] : ' '),
                            (puzzle[i, j].isPossible(k + 1) ? Convert.ToString(k + 2)[0] : ' '),
                            (puzzle[i, j].isPossible(k + 2) ? Convert.ToString(k + 3)[0] : ' ')
                        );
                        if ((j + 1) % 3 == 0)
                        {
                            Console.Write(" ");
                        }
                    }
                    Console.WriteLine();
                }
                Console.WriteLine("+-------+ +-------+ +-------+  +-------+ +-------+ +-------+  +-------+ +-------+ +-------+");
                if ((i + 1) % 3 == 0)
                {
                    Console.WriteLine();
                }
            }
        }

        public void display(String header)
        {
            Console.WriteLine(header);
            displayPuzzle();
        }

        public void execute()
        {
            // Console defaults to a 80 character width???
            //Console.SetBufferSize(width: 120, height: 100);

            initialize();
            display("Start of Puzzle:");
            // solve();
        }
    }
}

namespace Sudoku.ConsoleApplication
{
    class Program
    {
        static void Main(string[] args)
        {
            Console.WriteLine("Hello World");
            Sudoko.SudokuPuzzle puzzle = new Sudoko.SudokuPuzzle();
            puzzle.execute();
        }
    }
}
