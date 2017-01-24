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

        //private int found;               // field
        public int Found { get; set; }   // property

        public bool IsSolved { get { return ( Found != SudokuCell.NOTFOUND ); } }  // derived property

        private bool[] possible = new bool[9];

        public bool isPossible(int i)
        {
            return possible[i];
        }

        public void setPossible(int i, bool value = true)
        {
            possible[i] = value;
        }

        public void setExclusivePossibles(int[] possibles)
        {
            for (int i= 0; i<possible.GetLength(0); i++)
            {
                possible[i] = false;
            }
            for (int i=0;i<possibles.GetLength(0); i++)
            {
                int value = possibles[i];
                possible[value] = true;
            }
        }

        public SudokuCell()
        {
            // allocate sudoku call
        }

        public void initialize()
        {
            // initialize sudoku call

            Found = NOTFOUND;

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

            //
            // read input string
            //

            String input = getInput();

            // string contains rows that are either contiguous or separated by a '+'
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
                        // Know that this cell has a solution, 
                        // but will let findNakedSingles() discover it and clean up neighbors
                        puzzle[row, column].setPossible(cellDigit, true);
                    }
                }
                inputIndex += inputIncrement;
            }
        }

        public void display()
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
            display();
        }

        //
        // puzzle[i, column] solved with value k
        // remove horizonal neighbors as a possibility for k
        //

        public bool removeHorizontalNeighbors(int i, int j, int k)
        {
            bool foundRemoval;

            foundRemoval = false;
            for (int column = 0; column < 9; column++)
            {
                bool isCellToChange = ( column != j );
                bool isChangeNeeded = puzzle[i, column].isPossible(k);
                if ( isCellToChange && isChangeNeeded )
                {
                    foundRemoval = true;
                    puzzle[i, column].setPossible(k, false);
                }
            }

            return foundRemoval;
        }

        //
        // puzzle[i, column] solved with value k
        // remove vertical neighbors as a possibility for k
        //

        public bool removeVerticalNeighbors(int i, int j, int k)
        {
            bool foundRemoval;

            foundRemoval = false;
            for (int row = 0; row < 9; row++)
            {
                bool isCellToChange = ( row != i );
                bool isChangeNeeded = puzzle[row, j].isPossible(k);
                if ( isCellToChange && isChangeNeeded )
                {
                    foundRemoval = true;
                    puzzle[row, j].setPossible(k, false);
                }
            }

            return foundRemoval;
        }

        //
        // puzzle[i, column] solved with value k
        // remove grid neighbors as a possibility for k
        //

        public bool removeGridNeighbors(int i, int j, int k)
        {
            int starti, startj;
            bool foundRemoval;

            foundRemoval = false;
            starti = i / 3 * 3;
            startj = j / 3 * 3;
            for (int ii = starti; ii < starti + 3; ii++)
            {
                for (int jj = startj; jj < startj + 3; jj++)
                {
                    bool isCellToChange = ( ii != i || jj != j );
                    bool isChangeNeeded = puzzle[ii, jj].isPossible(k);
                    if ( isCellToChange && isChangeNeeded )
                    {
                        foundRemoval = true;
                        puzzle[ii, jj].setPossible(k, false);
                    }
                }
            }

            return foundRemoval;
        }

        //
        // puzzle[i, column] solved with value k
        // remove all neighbors as a possibility for k
        //

        public bool removeNeighbors(int i, int j, int k)
        {
            //display(String.Format("before remove neighbors for {0} at cell({1},{2})", k+1, i+1, j+1));
            bool foundRemoval = false;
            foundRemoval |= removeHorizontalNeighbors(i, j, k);
            foundRemoval |= removeVerticalNeighbors(i, j, k);
            foundRemoval |= removeGridNeighbors(i, j, k);
            //display(String.Format("after remove neighbors for {0} at cell({1},{2})", k + 1, i + 1, j + 1));
            return foundRemoval;
        }

        //
        // find cells where there is only one possible value left
        //

        public bool findNakedSingles()
        {
            int iNumPossibilities;
            bool foundSingle;
            bool foundSingles;
            int iLastFound;
            int[,] Singles = new int[9,9];

            foundSingles = false;
            for (int i = 0; i < 9; i++)
            {
                for (int j = 0; j < 9; j++)
                {
                    if ( !puzzle[i, j].IsSolved )
                    {
                        iNumPossibilities = 0;
                        iLastFound = SudokuCell.NOTFOUND;
                        for (int k = 0; k < 9; k++)
                        {
                            if ( puzzle[i, j].isPossible(k) )
                            {
                                iLastFound = k;
                                iNumPossibilities++;
                                if (iNumPossibilities > 1) break;
                            }
                        }
                        foundSingle = ( iNumPossibilities == 1 );
                        if ( foundSingle )
                        {
                            foundSingles = true;
                            Singles[i, j] = iLastFound;
                        }
                        else
                        {
                            Singles[i, j] = SudokuCell.NOTFOUND;
                        }

                    }
                    else
                    {
                        Singles[i, j] = SudokuCell.NOTFOUND;
                    }
                }
            }

            if ( foundSingles )
            {
                for (int i = 0; i < 9; i++)
                {
                    for (int j = 0; j < 9; j++)
                    {
                        foundSingle = ( Singles[i, j] != SudokuCell.NOTFOUND );
                        if ( foundSingle )
                        {
                            int k = Singles[i, j];
                            // mark cell as a single
                            Console.WriteLine("found naked single {0} at cell({1},{2})", k + 1, i + 1, j + 1);
                            solveCell(i, j, k);
                        }
                    }
                }
            }

            return foundSingles;
        }

        public bool findHiddenSinglesHorizonatlNeighbors()
        {
            bool foundHiddenSingle = false;

            for (int row = 0; row < 9; row++)
            {
                for (int k = 0; k < 9; k++)
                {
                    int numPossibilities = 0;
                    int lastFoundColumn = SudokuCell.NOTFOUND;
                    for (int column = 0; column < 9; column++)
                    {
                        if ( puzzle[row, column].isPossible(k) )
                        {
                            numPossibilities++;
                            lastFoundColumn = column;
                            if ( numPossibilities > 1 ) break;
                        }
                    }
                    bool hiddenSingle = ( numPossibilities == 1 );
                    if ( hiddenSingle )
                    {
                        Boolean isSingleAlready = ( puzzle[row, lastFoundColumn].IsSolved );
                        if (!isSingleAlready)
                        {
                            foundHiddenSingle = true;
                            Console.WriteLine("found hidden single {0} in row {1}", k + 1, row + 1);
                            solveCell(row, lastFoundColumn, k);
                        }
                    }
                }
            }

            return foundHiddenSingle;
        }

        public bool findHiddenSinglesVerticalNeighbors()
        {
            bool foundHiddenSingle = false;

            for (int column = 0; column < 9; column++)
            {
                for (int k = 0; k < 9; k++)
                {
                    int numPossibilities = 0;
                    int lastRowFound = SudokuCell.NOTFOUND;
                    for (int row = 0; row < 9; row++)
                    {
                        if (puzzle[row, column].isPossible(k))
                        {
                            numPossibilities++;
                            lastRowFound = row;
                            if ( numPossibilities > 1 ) break;
                        }
                    }
                    bool hiddenSingle = ( numPossibilities == 1 );
                    if (hiddenSingle)
                    {
                        Boolean isSingleAlready = (puzzle[lastRowFound, column].IsSolved);
                        if ( !isSingleAlready )
                        {
                            foundHiddenSingle = true;
                            Console.WriteLine("found hidden single {0} in column {1}", k+1, column+1);
                            solveCell(lastRowFound, column, k);
                        }
                    }
                }
            }

            return foundHiddenSingle;
        }

        public bool findHiddenSinglesGridNeighbors()
        {
            bool foundHiddenSingle = false;

            for (int gridi = 0; gridi < 3; gridi++)
            {
                for (int gridj = 0; gridj < 3; gridj++)
                {
                    int starti = gridi * 3;
                    int startj = gridj * 3;
                    for (int k = 0; k < 9; k++)
                    {
                        int lastRowFound    = SudokuCell.NOTFOUND;
                        int lastCoulmnFound = SudokuCell.NOTFOUND;
                        int numPossibilities = 0;
                        for (int row = starti; row < starti + 3; row++)
                        {
                            for (int column = startj; column < startj + 3; column++)
                            {
                                if ( puzzle[row, column].isPossible(k) )
                                {
                                    numPossibilities++;
                                    lastRowFound = row;
                                    lastCoulmnFound = column;
                                    if ( numPossibilities > 1 ) break;
                                }
                            }
                            if ( numPossibilities > 1 ) break;
                        }
                        bool foundSingle = ( numPossibilities == 1 );
                        if ( foundSingle )
                        {
                            bool isSingleAlready = ( puzzle[lastRowFound,lastCoulmnFound].IsSolved );
                            if ( !isSingleAlready )
                            {
                                foundHiddenSingle = true;
                                Console.WriteLine("found hidden single {0} in cell({1},{2}) in grid({3}...{4},{5}...{6})",
                                    k + 1, lastRowFound + 1, lastCoulmnFound + 1, starti + 1, starti + 3, startj + 1, startj + 3);
                                solveCell(lastRowFound, lastCoulmnFound, k);
                            }
                        }
                    }
                }
            }

            return foundHiddenSingle;
        }

        //
        // find a single occurence of value in row, column or grid
        // where the value is not the only possibility in its cell (aka it is hidden)
        //

        public bool findHiddenSingles()
        {
            bool foundHiddenSingle;
            foundHiddenSingle = false;
            foundHiddenSingle |= findHiddenSinglesHorizonatlNeighbors();
            foundHiddenSingle |= findHiddenSinglesVerticalNeighbors();
            foundHiddenSingle |= findHiddenSinglesGridNeighbors();
            return foundHiddenSingle;
        }

        public void solveCell(int i, int j, int k)
        {
            puzzle[i, j].Found = k;
            puzzle[i, j].setExclusivePossibles(new int[] { k }); // cell may need to remove other possibilites
            removeNeighbors(i, j, k);
        }

        public void solve()
        {
            bool didSomething;

            do
            {
                didSomething = findNakedSingles();
                if (!didSomething)
                {
                    didSomething = findHiddenSingles();
                    if (!didSomething)
                    {
/*                        didSomething = findNakedPairs();
                        if (!didSomething)
                        {
                            didSomething |= findLockedCandidate();
                        } */
                    }
                } 
            } while ( didSomething );
        }

        public void execute()
        {
            // Console defaults to a 80 character width???
            //Console.SetBufferSize(width: 120, height: 100);

            initialize();
            display("Start of Puzzle:");
            solve();
            display("End of Puzzle:");
        }
    }
}

namespace Sudoku.ConsoleApplication
{
    class Program
    {
        static void Main(string[] args)
        {
            Sudoko.SudokuPuzzle puzzle = new Sudoko.SudokuPuzzle();
            puzzle.execute();
        }
    }
}
