using System;
using System.Linq;

namespace Sudoko
{
    class SudokuCell
    {
        public const int NOTFOUND = -1;  // a const object is always static by definition

        //private int found;             // field
        public int Found { get; set; }   // property

        public bool IsSolved { get { return (Found != SudokuCell.NOTFOUND); } }  // derived property

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
            for (int i = 0; i < possible.GetLength(0); i++)
            {
                possible[i] = false;
            }
            for (int i = 0; i < possibles.GetLength(0); i++)
            {
                int value = possibles[i];
                possible[value] = true;
            }
        }

        public SudokuCell()
        {
            // allocate member possible above
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

        private String getGame()
        {
            return SudokuGames.getGame();
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

            String input = getGame();

            // string contains rows that are either contiguous or separated by a '+'
            int inputIncrement = (input[9] == '+' ? 10 : 9);
            int inputIndex = 0;

            for (int row = 0; row < 9; row++)
            {
                String inputRow = input.Substring(inputIndex, 9);
                for (int column = 0; column < 9; column++)
                {
                    Char cell = inputRow[column];

                    // when the input string does not have a solution for sudoku cell,
                    // value will be either '0' or '.' (how other programs write puzzles to stdout)
                    // normalize to '.' hre
                    if (cell == '0') cell = '.';

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
                bool isCellToChange = (column != j);
                bool isChangeNeeded = puzzle[i, column].isPossible(k);
                if (isCellToChange && isChangeNeeded)
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
                bool isCellToChange = (row != i);
                bool isChangeNeeded = puzzle[row, j].isPossible(k);
                if (isCellToChange && isChangeNeeded)
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
                    bool isCellToChange = (ii != i || jj != j);
                    bool isChangeNeeded = puzzle[ii, jj].isPossible(k);
                    if (isCellToChange && isChangeNeeded)
                    {
                        foundRemoval = true;
                        puzzle[ii, jj].setPossible(k, false);
                    }
                }
            }

            return foundRemoval;
        }

        //
        // puzzle[i, j] solved with value k
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
            int[,] Singles = new int[9, 9];

            foundSingles = false;
            for (int i = 0; i < 9; i++)
            {
                for (int j = 0; j < 9; j++)
                {
                    if (!puzzle[i, j].IsSolved)
                    {
                        iNumPossibilities = 0;
                        iLastFound = SudokuCell.NOTFOUND;
                        for (int k = 0; k < 9; k++)
                        {
                            if (puzzle[i, j].isPossible(k))
                            {
                                iLastFound = k;
                                iNumPossibilities++;
                                if (iNumPossibilities > 1) break;
                            }
                        }
                        foundSingle = (iNumPossibilities == 1);
                        if (foundSingle)
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

            if (foundSingles)
            {
                for (int i = 0; i < 9; i++)
                {
                    for (int j = 0; j < 9; j++)
                    {
                        foundSingle = (Singles[i, j] != SudokuCell.NOTFOUND);
                        if (foundSingle)
                        {
                            int k = Singles[i, j];
                            // mark cell as a single
                            log(String.Format("found naked single {0} at cell({1},{2})", k + 1, i + 1, j + 1));
                            solveCell(i, j, k);
                        }
                    }
                }
            }

            return foundSingles;
        }

        public bool findLockedCandidateHorizontal()
        {
            int numChanges;
            bool[,,] Candidate = new bool[9, 3, 9];

            numChanges = 0;

            for (int row = 0; row < 9; row++)
            {
                for (int k = 0; k < 9; k++)
                {
                    Candidate[row, 0, k] = false;
                    Candidate[row, 1, k] = false;
                    Candidate[row, 2, k] = false;
                }
                for (int k = 0; k < 9; k++)
                {
                    for (int column = 0; column < 9; column++)
                    {
                        if (!puzzle[row, column].IsSolved)
                        {
                            int gridj = column / 3;
                            Candidate[row, gridj, k] |= puzzle[row, column].isPossible(k);
                        }
                    }
                }
            }

            //
            // Locked Candidate Type 1
            // if single is in the grid's row/column but is not in the grid's other row/column, 
            // then we can exclude the single from the rest of the row/column
            //

            for (int gridi = 0; gridi < 3; gridi++)
            {
                for (int gridj = 0; gridj < 3; gridj++)
                {
                    // Console.WriteLine("grid({0},{1})", gridi+1, gridj+1);
                    int starti = gridi * 3;
                    int startj = gridj * 3;
                    for (int k = 0; k < 9; k++)
                    {
                        bool foundLockedCandidate = false;
                        int lockedRow = SudokuCell.NOTFOUND; // C# quesses use below is "unassigned" and thus an error
                        if (Candidate[starti, gridj, k] && !Candidate[starti + 1, gridj, k] && !Candidate[starti + 2, gridj, k])
                        {
                            foundLockedCandidate = true;
                            lockedRow = starti;
                        }
                        if (!Candidate[starti, gridj, k] && Candidate[starti + 1, gridj, k] && !Candidate[starti + 2, gridj, k])
                        {
                            foundLockedCandidate = true;
                            lockedRow = starti + 1;
                        }
                        if (!Candidate[starti, gridj, k] && !Candidate[starti + 1, gridj, k] && Candidate[starti + 2, gridj, k])
                        {
                            foundLockedCandidate = true;
                            lockedRow = starti + 2;
                        }
                        if (foundLockedCandidate)
                        {
                            // row includes Candidate[lockedRow, 0, k] + Candidate[lockedRow, 1, k] +  Candidate[lockedRow, 2, k]
                            bool foundPossibilitiesToRemove = (
                                  (gridj != 0 && Candidate[lockedRow, 0, k]) ||
                                  (gridj != 1 && Candidate[lockedRow, 1, k]) ||
                                  (gridj != 2 && Candidate[lockedRow, 2, k])
                                );
                            if (foundPossibilitiesToRemove)
                            {
                                log(String.Format("found type 1 locked candidated {0} in row cells({1},{2}..{3}), exclude the rest of the row",
                                                  k + 1, lockedRow + 1, startj + 1, startj + 3));
                                for (int column = 0; column < 9; column++)
                                {
                                    int thisGridColumn = column / 3;
                                    bool gridToExclude = (thisGridColumn == gridj);
                                    if (!gridToExclude)
                                    {
                                        if (puzzle[lockedRow, column].isPossible(k))
                                        {
                                            numChanges++;
                                            puzzle[lockedRow, column].setPossible(k, false);
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

            for (int row = 0; row < 9; row++)
            {
                for (int k = 0; k < 9; k++)
                {
                    bool foundLockedCandidate = false;
                    int startj = SudokuCell.NOTFOUND; // C# guesses use below is "unassigned" and thus an error
                    if (Candidate[row, 0, k] && !Candidate[row, 1, k] && !Candidate[row, 2, k])
                    {
                        foundLockedCandidate = true;
                        startj = 0;
                    }
                    if (!Candidate[row, 0, k] && Candidate[row, 1, k] && !Candidate[row, 2, k])
                    {
                        foundLockedCandidate = true;
                        startj = 3;
                    }
                    if (!Candidate[row, 0, k] && !Candidate[row, 1, k] && Candidate[row, 2, k])
                    {
                        foundLockedCandidate = true;
                        startj = 6;
                    }
                    if (foundLockedCandidate)
                    {
                        int gridj = startj / 3;
                        int starti = row / 3 * 3;
                        // grid includes Candidate[starti, gridj, k] + Candidate[starti+1, gridj, k] + Candidate[starti+2, gridj, k]
                        bool foundPossibilitiesToRemove = (
                              (starti != row && Candidate[starti, gridj, k]) ||
                              (starti + 1 != row && Candidate[starti + 1, gridj, k]) ||
                              (starti + 2 != row && Candidate[starti + 2, gridj, k])
                            );
                        if (foundPossibilitiesToRemove)
                        {
                            log(String.Format("found type 2 locked candidated {0} in row cells({1},{2}..{3}), exclude the rest of the grid",
                                              k + 1, row + 1, startj + 1, startj + 3));

                            for (int i = starti; i < starti + 3; i++)
                            {
                                bool excludeLockedRow = (i == row);
                                if (!excludeLockedRow)
                                {
                                    for (int j = startj; j < startj + 3; j++)
                                    {
                                        if (puzzle[i, j].isPossible(k))
                                        {
                                            numChanges++;
                                            puzzle[i, j].setPossible(k, false);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            return (numChanges > 0);
        }

        public bool findLockedCandidateVertical()
        {
            int numChanges;
            bool[,,] Candidate = new bool[9, 3, 9];

            numChanges = 0;

            for (int column = 0; column < 9; column++)
            {
                for (int k = 0; k < 9; k++)
                {
                    Candidate[column, 0, k] = false;
                    Candidate[column, 1, k] = false;
                    Candidate[column, 2, k] = false;
                }
                for (int k = 0; k < 9; k++)
                {
                    for (int row = 0; row < 9; row++)
                    {
                        if (!puzzle[row, column].IsSolved)
                        {
                            int gridi = row / 3;
                            Candidate[column, gridi, k] |= puzzle[row, column].isPossible(k);
                        }
                    }
                }
            }

            //
            // Locked Candidate Type 1
            // if single is in the grid's row/column but is not in the grid's other row/column, 
            // then we can exclude the single from the rest of the row/column
            //

            for (int gridi = 0; gridi < 3; gridi++)
            {
                for (int gridj = 0; gridj < 3; gridj++)
                {
                    // Console.WriteLine("grid({0},{1})", gridi+1, gridj+1);
                    int starti = gridi * 3;
                    int startj = gridj * 3;
                    for (int k = 0; k < 9; k++)
                    {
                        bool foundLockedCandidate = false;
                        int lockedColumn = SudokuCell.NOTFOUND; // C# guesses use below is "unassigned" and thus an error
                        if (Candidate[startj, gridi, k] && !Candidate[startj + 1, gridi, k] && !Candidate[startj + 2, gridi, k])
                        {
                            foundLockedCandidate = true;
                            lockedColumn = startj;
                        }
                        if (!Candidate[startj, gridi, k] && Candidate[startj + 1, gridi, k] && !Candidate[startj + 2, gridi, k])
                        {
                            foundLockedCandidate = true;
                            lockedColumn = startj + 1;
                        }
                        if (!Candidate[startj, gridi, k] && !Candidate[startj + 1, gridi, k] && Candidate[startj + 2, gridi, k])
                        {
                            foundLockedCandidate = true;
                            lockedColumn = startj + 2;
                        }
                        if (foundLockedCandidate)
                        {
                            // row includes Candidate[lockedRow, 0, k] + Candidate[lockedRow, 1, k] +  Candidate[lockedRow, 2, k]
                            bool foundPossibilitiesToRemove = (
                                  (gridi != 0 && Candidate[lockedColumn, 0, k]) ||
                                  (gridi != 1 && Candidate[lockedColumn, 1, k]) ||
                                  (gridi != 2 && Candidate[lockedColumn, 2, k])
                                );
                            if (foundPossibilitiesToRemove)
                            {
                                log(String.Format("found type 1 locked candidated {0} in column cells({1}..{2},{3}), exclude the rest of the column",
                                                  k + 1, starti + 1, starti + 3, lockedColumn + 1));
                                for (int row = 0; row < 9; row++)
                                {
                                    int thisGridRow = row / 3;
                                    bool gridToExclude = (thisGridRow == gridi);
                                    if (!gridToExclude)
                                    {
                                        if (puzzle[row, lockedColumn].isPossible(k))
                                        {
                                            numChanges++;
                                            puzzle[row, lockedColumn].setPossible(k, false);
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

            for (int column = 0; column < 9; column++)
            {
                for (int k = 0; k < 9; k++)
                {
                    bool foundLockedCandidate = false;
                    int starti = SudokuCell.NOTFOUND; // C# quesses use below is "unassigned" and thus an error
                    if (Candidate[column, 0, k] && !Candidate[column, 1, k] && !Candidate[column, 2, k])
                    {
                        foundLockedCandidate = true;
                        starti = 0;
                    }
                    if (!Candidate[column, 0, k] && Candidate[column, 1, k] && !Candidate[column, 2, k])
                    {
                        foundLockedCandidate = true;
                        starti = 3;
                    }
                    if (!Candidate[column, 0, k] && !Candidate[column, 1, k] && Candidate[column, 2, k])
                    {
                        foundLockedCandidate = true;
                        starti = 6;
                    }
                    if (foundLockedCandidate)
                    {
                        int gridi = starti / 3;
                        int startj = column / 3 * 3;
                        // grid includes Candidate[starti, gridj, k] + Candidate[starti+1, gridj, k] + Candidate[starti+2, gridj, k]
                        bool foundPossibilitiesToRemove = (
                              (startj != column && Candidate[startj, gridi, k]) ||
                              (startj + 1 != column && Candidate[startj + 1, gridi, k]) ||
                              (startj + 2 != column && Candidate[startj + 2, gridi, k])
                            );
                        if (foundPossibilitiesToRemove)
                        {
                            log(String.Format("found type 2 locked candidated {0} in column cells({1}..{2},{3}), exclude the rest of the grid",
                                              k + 1, starti + 1, starti + 3, column + 1));

                            for (int j = startj; j < startj + 3; j++)
                            {
                                bool excludeLockedColumn = (j == column);
                                if (!excludeLockedColumn)
                                {
                                    for (int i = starti; i < starti + 3; i++)
                                    {
                                        if (puzzle[i, j].isPossible(k))
                                        {
                                            numChanges++;
                                            puzzle[i, j].setPossible(k, false);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            return (numChanges > 0);
        }

        //
        // Locked Candidate Type 1
        // if single is in the grid's row/column but is not in the grid's other row/column, 
        // then we can exclude the single from the rest of the row/column
        //
        // Locked Candidate Type 2
        // if the single is in the grid's row/column but not in the rest of the row/column,
        // then we can restrict the single from the rest of the grid
        //

        public bool findLockedCandidate()
        {
            bool didSomething = false;
            didSomething |= findLockedCandidateHorizontal();
            didSomething |= findLockedCandidateVertical();
            return didSomething;
        }

        public struct CellCoordinate
        {
            public int row;
            public int column;
        }

        public delegate int SudokuAssist(CellCoordinate[] cells, string description);

        public bool findHorizontalExclusions(SudokuAssist assist)
        {
            int numChanges;
            CellCoordinate[] cells = new CellCoordinate[9];

            numChanges = 0;

            for (int row = 0; row < 9; row++)
            {
                for (int column = 0; column < 9; column++)
                {
                    cells[column] = new CellCoordinate { row = row, column = column };
                }
                numChanges += assist(cells, String.Format("for row {0}", row + 1));
            }

            return (numChanges > 0);
        }

        public bool findVerticalExclusions(SudokuAssist assist)
        {
            int numChanges;
            CellCoordinate[] cells = new CellCoordinate[9];

            numChanges = 0;

            for (int column = 0; column < 9; column++)
            {
                for (int row = 0; row < 9; row++)
                {
                    cells[row] = new CellCoordinate { row = row, column = column };
                }
                numChanges += assist(cells, String.Format("for column {0}", column + 1));
            }

            return (numChanges > 0);
        }

        public bool findGridExclusions(SudokuAssist assist)
        {
            int numChanges;
            CellCoordinate[] cells = new CellCoordinate[9];

            numChanges = 0;

            for (int gridi = 0; gridi < 3; gridi++)
            {
                for (int gridj = 0; gridj < 3; gridj++)
                {
                    int starti = gridi * 3;
                    int startj = gridj * 3;

                    int gridIndex = 0;
                    for (int row = starti; row < starti + 3; row++)
                    {
                        for (int column = startj; column < startj + 3; column++)
                        {
                            cells[gridIndex++] = new CellCoordinate { row = row, column = column };
                        }
                    }
                    numChanges += assist(cells, String.Format("for grid({0},{1})", gridi + 1, gridj + 1));
                }
            }

            return (numChanges > 0);
        }

        public bool findExclusions(SudokuAssist assist)
        {
            bool foundExclusions = false;
            foundExclusions |= findHorizontalExclusions(assist);
            foundExclusions |= findVerticalExclusions(assist);
            foundExclusions |= findGridExclusions(assist);
            return foundExclusions;
        }

        public int findNakedPairsAssist(CellCoordinate[] cells, string description)
        {
            int numChanges;
            int numCandidatePairs;
            int[,] CandidatePairs = new int[9, 3];
            int numNakedPairs;
            int[,] NakedPairs = new int[9, 4];

            numChanges = 0;

            numCandidatePairs = 0;
            for (int c = 0; c < cells.Length; c++)
            {
                int row = cells[c].row;
                int column = cells[c].column;

                int p1 = SudokuCell.NOTFOUND;
                int p2 = SudokuCell.NOTFOUND;
                int numPossibilities = 0;

                for (int k = 0; k < 9; k++)
                {
                    if (!puzzle[row, column].IsSolved)
                    {
                        if (puzzle[row, column].isPossible(k))
                        {
                            numPossibilities++;
                            if (numPossibilities > 2) break;
                            if (p1 == SudokuCell.NOTFOUND) p1 = k;
                            else p2 = k;
                        }
                    }
                }

                if (numPossibilities == 2)
                {
                    // we have found a pair candidate
                    CandidatePairs[numCandidatePairs, 0] = p1;
                    CandidatePairs[numCandidatePairs, 1] = p2;
                    CandidatePairs[numCandidatePairs, 2] = c;
                    numCandidatePairs++;
                }
            }

            numNakedPairs = 0;
            if (numCandidatePairs >= 2)
            {
                for (int c1 = 0; c1 < numCandidatePairs; c1++)
                {
                    for (int c2 = c1 + 1; c2 < numCandidatePairs; c2++)
                    {
                        int p1, p2;
                        bool[] PossiblesForPair = new bool[9] { false, false, false, false, false, false, false, false, false };

                        p1 = CandidatePairs[c1, 0];
                        p2 = CandidatePairs[c1, 1];
                        PossiblesForPair[p1] = true;
                        PossiblesForPair[p2] = true;

                        p1 = CandidatePairs[c2, 0];
                        p2 = CandidatePairs[c2, 1];
                        PossiblesForPair[p1] = true;
                        PossiblesForPair[p2] = true;

                        int numPossiblesForPair = 0;
                        // alernative implementation: lamba
                        numPossiblesForPair = PossiblesForPair.Count(k => k == true);

                        numPossiblesForPair = 0;
                        p1 = SudokuCell.NOTFOUND;
                        p2 = SudokuCell.NOTFOUND;
                        for (int k = 0; k < 9; k++)
                        {
                            if (PossiblesForPair[k])
                            {
                                numPossiblesForPair++;
                                if (p1 == SudokuCell.NOTFOUND) p1 = k;
                                else if (p2 == SudokuCell.NOTFOUND) p2 = k;
                            }
                        }
                        if (numPossiblesForPair == 2)
                        {
                            // we have a pair ( p1 p2 )
                            NakedPairs[numNakedPairs, 0] = p1;
                            NakedPairs[numNakedPairs, 1] = p2;
                            NakedPairs[numNakedPairs, 2] = CandidatePairs[c1, 2]; // index into cells[]
                            NakedPairs[numNakedPairs, 3] = CandidatePairs[c2, 2];
                            numNakedPairs++;
                        }
                    }
                }
            }

            if (numNakedPairs > 0)
            {
                for (int t = 0; t < numNakedPairs; t++)
                {
                    int p1 = NakedPairs[t, 0];
                    int p2 = NakedPairs[t, 1];
                    int c1 = NakedPairs[t, 2];
                    int c2 = NakedPairs[t, 3];

                    int numChangesForPair = 0;
                    for (int c = 0; c < cells.Length; c++)
                    {
                        bool cellOutsideOfTriple = (c != c1 && c != c2);
                        if (cellOutsideOfTriple)
                        {
                            int row = cells[c].row;
                            int column = cells[c].column;
                            if (!puzzle[row, column].IsSolved)
                            {
                                if (puzzle[row, column].isPossible(p1))
                                {
                                    puzzle[row, column].setPossible(p1, false);
                                    numChangesForPair++;
                                    numChanges++;
                                }
                                if (puzzle[row, column].isPossible(p2))
                                {
                                    puzzle[row, column].setPossible(p2, false);
                                    numChangesForPair++;
                                    numChanges++;
                                }
                            }
                        }
                    }
                    if (numChangesForPair > 0)
                    {
                        log(String.Format("found naked pair ( {0} {1} ) {2}", p1 + 1, p2 + 1, description));
                    }
                    else
                    {
                        log(String.Format("xxx found naked pair ( {0} {1} ) {2}", p1 + 1, p2 + 1, description));
                    }
                }
            }

            return numChanges;
        }

        //
        // find naked pair cells in row, column, or grid where both cells only have the exact two possibilities.
        // other cells in the row, column, or grid can then eliminate those two possibilities
        //

        public bool findNakedPairs()
        {
            return findExclusions(findNakedPairsAssist);
        }

        public int findNakedTriplesAssist(CellCoordinate[] cells, string description)
        {
            int numChanges;
            int numCandidateTriples;
            int[,] CandidateTriples = new int[9, 4];
            int numNakedTriples;
            int[,] NakedTriples = new int[9, 6];

            numChanges = 0;

            numCandidateTriples = 0;
            for (int c = 0; c < cells.Length; c++)
            {
                int row = cells[c].row;
                int column = cells[c].column;

                int t1 = SudokuCell.NOTFOUND;
                int t2 = SudokuCell.NOTFOUND;
                int t3 = SudokuCell.NOTFOUND;
                int numPossibilities = 0;

                for (int k = 0; k < 9; k++)
                {
                    if (!puzzle[row, column].IsSolved)
                    {
                        if (puzzle[row, column].isPossible(k))
                        {
                            numPossibilities++;
                            if (numPossibilities > 3) break;
                            if (t1 == SudokuCell.NOTFOUND) t1 = k;
                            else if (t2 == SudokuCell.NOTFOUND) t2 = k;
                            else t3 = k;
                        }
                    }
                }

                if (numPossibilities == 3 || numPossibilities == 2)
                {
                    // we have found a triple candidate
                    CandidateTriples[numCandidateTriples, 0] = t1;
                    CandidateTriples[numCandidateTriples, 1] = t2;
                    CandidateTriples[numCandidateTriples, 2] = t3;  // which could be SudokuCell.NOTFOUND
                    CandidateTriples[numCandidateTriples, 3] = c;
                    numCandidateTriples++;
                }
            }

            numNakedTriples = 0;
            if (numCandidateTriples >= 3)
            {
                for (int c1 = 0; c1 < numCandidateTriples; c1++)
                {
                    for (int c2 = c1 + 1; c2 < numCandidateTriples; c2++)
                    {
                        for (int c3 = c2 + 1; c3 < numCandidateTriples; c3++)
                        {
                            int t1, t2, t3;
                            bool[] PossiblesForTriple = new bool[9] { false, false, false, false, false, false, false, false, false };

                            t1 = CandidateTriples[c1, 0];
                            t2 = CandidateTriples[c1, 1];
                            t3 = CandidateTriples[c1, 2];
                            PossiblesForTriple[t1] = true;
                            PossiblesForTriple[t2] = true;
                            if (t3 != SudokuCell.NOTFOUND)
                            {
                                PossiblesForTriple[t3] = true;
                            }

                            t1 = CandidateTriples[c2, 0];
                            t2 = CandidateTriples[c2, 1];
                            t3 = CandidateTriples[c2, 2];
                            PossiblesForTriple[t1] = true;
                            PossiblesForTriple[t2] = true;
                            if (t3 != SudokuCell.NOTFOUND)
                            {
                                PossiblesForTriple[t3] = true;
                            }

                            t1 = CandidateTriples[c3, 0];
                            t2 = CandidateTriples[c3, 1];
                            t3 = CandidateTriples[c3, 2];
                            PossiblesForTriple[t1] = true;
                            PossiblesForTriple[t2] = true;
                            if (t3 != SudokuCell.NOTFOUND)
                            {
                                PossiblesForTriple[t3] = true;
                            }

                            int numPossiblesForTriple = 0;
                            // alernative implementation: lamba
                            numPossiblesForTriple = PossiblesForTriple.Count(k => k == true);

                            numPossiblesForTriple = 0;
                            t1 = SudokuCell.NOTFOUND;
                            t2 = SudokuCell.NOTFOUND;
                            t3 = SudokuCell.NOTFOUND;
                            for (int k = 0; k < 9; k++)
                            {
                                if (PossiblesForTriple[k])
                                {
                                    numPossiblesForTriple++;
                                    if (t1 == SudokuCell.NOTFOUND) t1 = k;
                                    else if (t2 == SudokuCell.NOTFOUND) t2 = k;
                                    else if (t3 == SudokuCell.NOTFOUND) t3 = k;
                                }
                            }
                            if (numPossiblesForTriple == 3)
                            {
                                // we have a triple ( t1 t2 t3 )
                                NakedTriples[numNakedTriples, 0] = t1;
                                NakedTriples[numNakedTriples, 1] = t2;
                                NakedTriples[numNakedTriples, 2] = t3;
                                NakedTriples[numNakedTriples, 3] = CandidateTriples[c1, 3]; // index into cells[]
                                NakedTriples[numNakedTriples, 4] = CandidateTriples[c2, 3];
                                NakedTriples[numNakedTriples, 5] = CandidateTriples[c3, 3];
                                numNakedTriples++;
                            }
                        }
                    }
                }
            }

            if (numNakedTriples > 0)
            {
                for (int t = 0; t < numNakedTriples; t++)
                {
                    int t1 = NakedTriples[t, 0];
                    int t2 = NakedTriples[t, 1];
                    int t3 = NakedTriples[t, 2];
                    int c1 = NakedTriples[t, 3];
                    int c2 = NakedTriples[t, 4];
                    int c3 = NakedTriples[t, 5];

                    int numChangesForTriple = 0;
                    for (int c = 0; c < cells.Length; c++)
                    {
                        bool cellOutsideOfTriple = (c != c1 && c != c2 && c != c3);
                        if (cellOutsideOfTriple)
                        {
                            int row = cells[c].row;
                            int column = cells[c].column;
                            if (!puzzle[row, column].IsSolved)
                            {
                                if (puzzle[row, column].isPossible(t1))
                                {
                                    puzzle[row, column].setPossible(t1, false);
                                    numChangesForTriple++;
                                    numChanges++;
                                }
                                if (puzzle[row, column].isPossible(t2))
                                {
                                    puzzle[row, column].setPossible(t2, false);
                                    numChangesForTriple++;
                                    numChanges++;
                                }
                                if (puzzle[row, column].isPossible(t3))
                                {
                                    puzzle[row, column].setPossible(t3, false);
                                    numChangesForTriple++;
                                    numChanges++;
                                }
                            }
                        }
                    }
                    if (numChangesForTriple > 0)
                    {
                        log(String.Format("found naked triple ( {0} {1} {2} ) {3}", t1 + 1, t2 + 1, t3 + 1, description));
                    }
                    else
                    {
                        log(String.Format("xxx found naked triple ( {0} {1} {2} ) {3}", t1 + 1, t2 + 1, t3 + 1, description));
                    }
                }
            }

            return numChanges;
        }

        //
        // find naked triple cells in row, column, or grid where both cells only have the a subset of three possibilities.
        // other cells in the row, column, or grid can then eliminate those three possibilities
        //

        public bool findNakedTriples()
        {
            return findExclusions(findNakedTriplesAssist);
        }

        public int findNakedQuadsAssist(CellCoordinate[] cells, string description)
        {
            int numChanges;
            int numCandidateQuads;
            int[,] CandidateQuads = new int[9, 5];
            int numNakedQuads;
            int[,] NakedQuads = new int[9, 8];

            numChanges = 0;

            numCandidateQuads = 0;
            for (int c = 0; c < cells.Length; c++)
            {
                int row = cells[c].row;
                int column = cells[c].column;

                int t1 = SudokuCell.NOTFOUND;
                int t2 = SudokuCell.NOTFOUND;
                int t3 = SudokuCell.NOTFOUND;
                int t4 = SudokuCell.NOTFOUND;
                int numPossibilities = 0;

                for (int k = 0; k < 9; k++)
                {
                    if (!puzzle[row, column].IsSolved)
                    {
                        if (puzzle[row, column].isPossible(k))
                        {
                            numPossibilities++;
                            if (numPossibilities > 4) break;
                            if (t1 == SudokuCell.NOTFOUND) t1 = k;
                            else if (t2 == SudokuCell.NOTFOUND) t2 = k;
                            else if (t3 == SudokuCell.NOTFOUND) t3 = k;
                            else t4 = k;
                        }
                    }
                }

                if (numPossibilities == 4 || numPossibilities == 3 || numPossibilities == 2)
                {
                    // we have found a quad candidate
                    CandidateQuads[numCandidateQuads, 0] = t1;
                    CandidateQuads[numCandidateQuads, 1] = t2;
                    CandidateQuads[numCandidateQuads, 2] = t3;  // which could be SudokuCell.NOTFOUND
                    CandidateQuads[numCandidateQuads, 3] = t4;  // which could be SudokuCell.NOTFOUND
                    CandidateQuads[numCandidateQuads, 4] = c;
                    numCandidateQuads++;
                }
            }

            numNakedQuads = 0;
            if (numCandidateQuads >= 4)
            {
                for (int c1 = 0; c1 < numCandidateQuads; c1++)
                {
                    for (int c2 = c1 + 1; c2 < numCandidateQuads; c2++)
                    {
                        for (int c3 = c2 + 1; c3 < numCandidateQuads; c3++)
                        {
                            for (int c4 = c3 + 1; c4 < numCandidateQuads; c4++)
                            {
                                int t1, t2, t3, t4;
                                bool[] PossiblesForQuad = new bool[9] { false, false, false, false, false, false, false, false, false };

                                t1 = CandidateQuads[c1, 0];
                                t2 = CandidateQuads[c1, 1];
                                t3 = CandidateQuads[c1, 2];
                                t4 = CandidateQuads[c1, 3];
                                PossiblesForQuad[t1] = true;
                                PossiblesForQuad[t2] = true;
                                if (t3 != SudokuCell.NOTFOUND)
                                {
                                    PossiblesForQuad[t3] = true;
                                }
                                if (t4 != SudokuCell.NOTFOUND)
                                {
                                    PossiblesForQuad[t4] = true;
                                }

                                t1 = CandidateQuads[c2, 0];
                                t2 = CandidateQuads[c2, 1];
                                t3 = CandidateQuads[c2, 2];
                                t4 = CandidateQuads[c2, 3];
                                PossiblesForQuad[t1] = true;
                                PossiblesForQuad[t2] = true;
                                if (t3 != SudokuCell.NOTFOUND)
                                {
                                    PossiblesForQuad[t3] = true;
                                }
                                if (t4 != SudokuCell.NOTFOUND)
                                {
                                    PossiblesForQuad[t4] = true;
                                }

                                t1 = CandidateQuads[c3, 0];
                                t2 = CandidateQuads[c3, 1];
                                t3 = CandidateQuads[c3, 2];
                                t4 = CandidateQuads[c3, 3];
                                PossiblesForQuad[t1] = true;
                                PossiblesForQuad[t2] = true;
                                if (t3 != SudokuCell.NOTFOUND)
                                {
                                    PossiblesForQuad[t3] = true;
                                }
                                if (t4 != SudokuCell.NOTFOUND)
                                {
                                    PossiblesForQuad[t4] = true;
                                }

                                t1 = CandidateQuads[c4, 0];
                                t2 = CandidateQuads[c4, 1];
                                t3 = CandidateQuads[c4, 2];
                                t4 = CandidateQuads[c4, 3];
                                PossiblesForQuad[t1] = true;
                                PossiblesForQuad[t2] = true;
                                if (t3 != SudokuCell.NOTFOUND)
                                {
                                    PossiblesForQuad[t3] = true;
                                }
                                if (t4 != SudokuCell.NOTFOUND)
                                {
                                    PossiblesForQuad[t4] = true;
                                }

                                int numPossiblesForQuad = 0;
                                // alernative implementation: lamba
                                numPossiblesForQuad = PossiblesForQuad.Count(k => k == true);

                                numPossiblesForQuad = 0;
                                t1 = SudokuCell.NOTFOUND;
                                t2 = SudokuCell.NOTFOUND;
                                t3 = SudokuCell.NOTFOUND;
                                t4 = SudokuCell.NOTFOUND;
                                for (int k = 0; k < 9; k++)
                                {
                                    if (PossiblesForQuad[k])
                                    {
                                        numPossiblesForQuad++;
                                        if (t1 == SudokuCell.NOTFOUND) t1 = k;
                                        else if (t2 == SudokuCell.NOTFOUND) t2 = k;
                                        else if (t3 == SudokuCell.NOTFOUND) t3 = k;
                                        else if (t4 == SudokuCell.NOTFOUND) t4 = k;
                                    }
                                }
                                if (numPossiblesForQuad == 4)
                                {
                                    // we have a quad ( t1 t2 t3 t4)
                                    NakedQuads[numNakedQuads, 0] = t1;
                                    NakedQuads[numNakedQuads, 1] = t2;
                                    NakedQuads[numNakedQuads, 2] = t3;
                                    NakedQuads[numNakedQuads, 3] = t4;
                                    NakedQuads[numNakedQuads, 4] = CandidateQuads[c1, 4]; // index into cells[]
                                    NakedQuads[numNakedQuads, 5] = CandidateQuads[c2, 4];
                                    NakedQuads[numNakedQuads, 6] = CandidateQuads[c3, 4];
                                    NakedQuads[numNakedQuads, 7] = CandidateQuads[c4, 4];
                                    numNakedQuads++;
                                }
                            }
                        }
                    }
                }
            }

            if (numNakedQuads > 0)
            {
                for (int t = 0; t < numNakedQuads; t++)
                {
                    int t1 = NakedQuads[t, 0];
                    int t2 = NakedQuads[t, 1];
                    int t3 = NakedQuads[t, 2];
                    int t4 = NakedQuads[t, 3];
                    int c1 = NakedQuads[t, 4];
                    int c2 = NakedQuads[t, 5];
                    int c3 = NakedQuads[t, 6];
                    int c4 = NakedQuads[t, 7];

                    int numChangesForQuad = 0;
                    for (int c = 0; c < cells.Length; c++)
                    {
                        bool cellOutsideOfQuad = (c != c1 && c != c2 && c != c3 && c != c4);
                        if (cellOutsideOfQuad)
                        {
                            int row = cells[c].row;
                            int column = cells[c].column;
                            if (!puzzle[row, column].IsSolved)
                            {
                                if (puzzle[row, column].isPossible(t1))
                                {
                                    puzzle[row, column].setPossible(t1, false);
                                    numChangesForQuad++;
                                    numChanges++;
                                }
                                if (puzzle[row, column].isPossible(t2))
                                {
                                    puzzle[row, column].setPossible(t2, false);
                                    numChangesForQuad++;
                                    numChanges++;
                                }
                                if (puzzle[row, column].isPossible(t3))
                                {
                                    puzzle[row, column].setPossible(t3, false);
                                    numChangesForQuad++;
                                    numChanges++;
                                }
                                if (puzzle[row, column].isPossible(t4))
                                {
                                    puzzle[row, column].setPossible(t4, false);
                                    numChangesForQuad++;
                                    numChanges++;
                                }
                            }
                        }
                    }
                    if (numChangesForQuad > 0)
                    {
                        log(String.Format("found naked quad ( {0} {1} {2} {3} ) {4}", t1 + 1, t2 + 1, t3 + 1, t4 + 1, description));
                    }
                    else
                    {
                        log(String.Format("xxx found naked quad ( {0} {1} {2} {3} ) {4}", t1 + 1, t2 + 1, t3 + 1, t4 + 1, description));
                    }
                }
            }

            return numChanges;
        }

        //
        // find naked quad cells in row, column, or grid where four cells only have the a subset of four possibilities.
        // other cells in the row, column, or grid can then eliminate those three possibilities
        //

        public bool findNakedQuads()
        {
            return findExclusions(findNakedQuadsAssist);
        }

        public int findHiddenSinglesAssist(CellCoordinate[] cells, string description)
        {
            int numChanges;

            numChanges = 0;

            for (int k = 0; k < 9; k++)
            {
                int numPossibilities = 0;
                int lastFound = SudokuCell.NOTFOUND;
                for (int c = 0; c < cells.Length; c++)
                {
                    int row = cells[c].row;
                    int column = cells[c].column;
                    if (!puzzle[row, column].IsSolved)
                    {
                        if (puzzle[row, column].isPossible(k))
                        {
                            numPossibilities++;
                            lastFound = c;
                        }
                    }
                }

                bool hiddenSingle = (numPossibilities == 1);
                if (hiddenSingle)
                {
                    int row = cells[lastFound].row;
                    int column = cells[lastFound].column;
                    log(String.Format("found hidden single {0} in cell({1},{2}) {3}", k + 1, row + 1, column + 1, description));
                    solveCell(row, column, k);
                    numChanges++;
                }
            }

            return numChanges;
        }

        //
        // find a single occurence of value in row, column or grid
        // where the value is not the only possibility in its cell (aka it is hidden)
        //

        public bool findHiddenSingles()
        {
            return findExclusions(findHiddenSinglesAssist);
        }

        public int findHiddenPairsAssist(CellCoordinate[] cells, string description)
        {
            int NumChanges;
            bool[] PossiblePairs = new bool[9];
            int[] NumPossiblePairs = new int[9];
            int[,] PossiblePairLocations = new int[9, 2];

            NumChanges = 0;

            for (int c = 0; c < cells.Length; c++)
            {
                int row = cells[c].row;
                int column = cells[c].column;

                if (!puzzle[row, column].IsSolved)
                {
                    for (int k = 0; k < 9; k++)
                    {
                        if (puzzle[row, column].isPossible(k))
                        {
                            if (NumPossiblePairs[k] < 2)
                            {
                                PossiblePairLocations[k, NumPossiblePairs[k]] = c; // index into cells[]
                            }
                            NumPossiblePairs[k]++;
                            PossiblePairs[k] = (NumPossiblePairs[k] == 2);
                        }
                    }
                }
            }

            for (int p1 = 0; p1 < 9; p1++)
            {
                if (PossiblePairs[p1])
                {
                    for (int p2 = p1 + 1; p2 < 9; p2++)
                    {
                        if (PossiblePairs[p2])
                        {
                            bool shareSameTwoCells = (
                                    PossiblePairLocations[p1, 0] == PossiblePairLocations[p2, 0] &&
                                    PossiblePairLocations[p1, 1] == PossiblePairLocations[p2, 1]
                                );
                            if (shareSameTwoCells)
                            {
                                int c1 = PossiblePairLocations[p1, 0];
                                int c2 = PossiblePairLocations[p1, 1];

                                // found a hidden pair ( p1 p2 ) in cells c1 and c2

                                int row1 = cells[c1].row;
                                int column1 = cells[c1].column;
                                int row2 = cells[c2].row;
                                int column2 = cells[c2].column;

                                // cells outside of c1 and c2, exclude p1 and p2 as possibilities

                                int numChangesThisPair = 0;
                                for (int k = 0; k < 9; k++)
                                {
                                    bool notInPair = (k != p1 && k != p2);
                                    if (notInPair)
                                    {
                                        if (puzzle[row1, column1].isPossible(k))
                                        {
                                            puzzle[row1, column1].setPossible(k, false);
                                            numChangesThisPair++;
                                            NumChanges++;
                                        }
                                        if (puzzle[row2, column2].isPossible(k))
                                        {
                                            puzzle[row2, column2].setPossible(k, false);
                                            numChangesThisPair++;
                                            NumChanges++;
                                        }
                                    }
                                }
                                if (numChangesThisPair > 0)
                                {
                                    log(String.Format("found hidden pair ( {0} {1} ) at cell({2},{3}) and cell({4},{5}), {6}",
                                        p1 + 1, p2 + 1, row1 + 1, column1 + 1, row2 + 1, column2 + 1, description));
                                }
                                else
                                {
                                    log(String.Format("xxx found hidden pair ( {0} {1} ) at cell({2},{3}) and cell({4},{5}), {6}",
                                        p1 + 1, p2 + 1, row1 + 1, column1 + 1, row2 + 1, column2 + 1, description));
                                }

                            }
                        }
                    }
                }
            }

            return NumChanges;

        }

        //
        // find hidden pair cells in row, column, or grid where both cells only have two possibilities.
        // from two hidden pair cells, remove possibilities from non-pairs
        //

        public bool findHiddenPairs()
        {
            return findExclusions(findHiddenPairsAssist);
        }

        public int findHiddenTriplesAssist(CellCoordinate[] cells, string description)
        {
            int NumChanges;
            bool[] PossibleTriples = new bool[9] { false, false, false, false, false, false, false, false, false };
            int[] NumPossibleTriples = new int[9] { 0, 0, 0, 0, 0, 0, 0, 0, 0 };
            int[,] PossibleTriplesLocations = new int[9, 3];

            NumChanges = 0;

            for (int c = 0; c < cells.Length; c++)
            {
                int row = cells[c].row;
                int column = cells[c].column;

                if (!puzzle[row, column].IsSolved)
                {
                    for (int k = 0; k < 9; k++)
                    {
                        if (puzzle[row, column].isPossible(k))
                        {
                            if (NumPossibleTriples[k] < 3)
                            {
                                PossibleTriplesLocations[k, NumPossibleTriples[k]] = c; // index into cells[]
                            }
                            NumPossibleTriples[k]++;
                            PossibleTriples[k] = (NumPossibleTriples[k] == 2 || NumPossibleTriples[k] == 3);
                        }
                    }
                }
            }

            NumChanges = 0;
            for (int t1 = 0; t1 < 9; t1++)
            {
                if (PossibleTriples[t1])
                {
                    for (int t2 = t1 + 1; t2 < 9; t2++)
                    {
                        if (PossibleTriples[t2])
                        {
                            for (int t3 = t2 + 1; t3 < 9; t3++)
                            {
                                if (PossibleTriples[t3])
                                {
                                    // at this point, we know that t1, t2, and t3 appears in two or three cells
                                    // need to determine if it is the same three.

                                    bool shareSameThreeCells;
                                    bool[] TripleCells = new bool[9] { false, false, false, false, false, false, false, false, false };

                                    TripleCells[PossibleTriplesLocations[t1, 0]] = true;
                                    TripleCells[PossibleTriplesLocations[t1, 1]] = true;
                                    if (NumPossibleTriples[t1] == 3)
                                    {
                                        TripleCells[PossibleTriplesLocations[t1, 2]] = true;
                                    }

                                    TripleCells[PossibleTriplesLocations[t2, 0]] = true;
                                    TripleCells[PossibleTriplesLocations[t2, 1]] = true;
                                    if (NumPossibleTriples[t2] == 3)
                                    {
                                        TripleCells[PossibleTriplesLocations[t2, 2]] = true;
                                    }

                                    TripleCells[PossibleTriplesLocations[t3, 0]] = true;
                                    TripleCells[PossibleTriplesLocations[t3, 1]] = true;
                                    if (NumPossibleTriples[t3] == 3)
                                    {
                                        TripleCells[PossibleTriplesLocations[t3, 2]] = true;
                                    }

                                    // lambda!
                                    int numTripleCells = TripleCells.Count(t => t == true);

                                    shareSameThreeCells = (numTripleCells == 3);
                                    if (shareSameThreeCells)
                                    {
                                        int c1 = SudokuCell.NOTFOUND;
                                        int c2 = SudokuCell.NOTFOUND;
                                        int c3 = SudokuCell.NOTFOUND;
                                        for (int c = 0; c < 9; c++)
                                        {
                                            if (TripleCells[c])
                                            {
                                                if (c1 == SudokuCell.NOTFOUND) c1 = c;
                                                else if (c2 == SudokuCell.NOTFOUND) c2 = c;
                                                else if (c3 == SudokuCell.NOTFOUND) c3 = c;
                                            }
                                        }

                                        // we have located a hidden triple ( t1 t2 t3 ) in cells c1, c2, and c3
                                        // all of the other possibilities in those three cells can be excluded

                                        int row1 = cells[c1].row;
                                        int column1 = cells[c1].column;
                                        int row2 = cells[c2].row;
                                        int column2 = cells[c2].column;
                                        int row3 = cells[c3].row;
                                        int column3 = cells[c3].column;

                                        int numChangesThisTriple = 0;
                                        for (int k = 0; k < 9; k++)
                                        {
                                            bool inTriple = (k == t1 || k == t2 || k == t3);
                                            bool notInTriple = (k != t1 && k != t2 && k != t3);
                                            if (notInTriple)
                                            {
                                                if (puzzle[row1, column1].isPossible(k))
                                                {
                                                    numChangesThisTriple++;
                                                    NumChanges++;
                                                    puzzle[row1, column1].setPossible(k, false);
                                                }
                                                if (puzzle[row2, column2].isPossible(k))
                                                {
                                                    numChangesThisTriple++;
                                                    NumChanges++;
                                                    puzzle[row2, column2].setPossible(k, false);
                                                }
                                                if (puzzle[row3, column3].isPossible(k))
                                                {
                                                    numChangesThisTriple++;
                                                    NumChanges++;
                                                    puzzle[row3, column3].setPossible(k, false);
                                                }
                                            }
                                        }

                                        if (numChangesThisTriple > 0)
                                        {
                                            log(String.Format("found hidden triple ( {0} {1} {2} ) at cell({3},{4}) and cell({5},{6}) and cell({7},{8}), {9}",
                                                t1 + 1, t2 + 1, t3 + 1, row1 + 1, column1 + 1, row2 + 1, column2 + 1, row3 + 1, column3 + 1, description));
                                        }
                                        else
                                        {
                                            log(String.Format("xxx found hidden triple ( {0} {1} {2} ) at cell({3},{4}) and cell({5},{6}) and cell({7},{8}), {9}",
                                                t1 + 1, t2 + 1, t3 + 1, row1 + 1, column1 + 1, row2 + 1, column2 + 1, row3 + 1, column3 + 1, description));
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            return NumChanges;
        }

        //
        // find hidden pair cells in row, column, or grid where both cells only have two possibilities.
        // from two hidden pair cells, remove possibilities from non-pairs
        //

        public bool findHiddenTriples()
        {
            return findExclusions(findHiddenTriplesAssist);
        }

        public void solveCell(int i, int j, int k)
        {
            puzzle[i, j].Found = k;
            puzzle[i, j].setExclusivePossibles(new int[] { k }); // other possibilites may need to be removed from cell
            removeNeighbors(i, j, k);
        }

        public void solve()
        {
            bool didSomething;

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

            do
            {
                // Easy
                didSomething = findNakedSingles();
                if (didSomething) continue;

                didSomething = findHiddenSingles();
                if (didSomething) continue;

                // Standard
                didSomething = findLockedCandidate();
                if (didSomething) continue;

                didSomething = findNakedPairs();
                if (didSomething) continue;

                // Hard
                didSomething = findNakedTriples();
                if (didSomething) continue;

                didSomething = findNakedQuads();
                if (didSomething) continue;

                // Harder
                didSomething = findHiddenPairs();
                if (didSomething) continue;

                didSomething = findHiddenTriples();
                if (didSomething) continue;

            } while (didSomething);
        }

        //
        // event callbackup function declaration
        //

        public delegate void logCallback(string msg);

        //
        // fireLogEvent event
        //

        public event logCallback fireLogEvent;

        //
        // Need a wrapper method to call the fireLogEvent,
        // to only fire event if there are subscribers.
        //

        public void log(string msg)
        {
            if (fireLogEvent != null)
            {
                fireLogEvent(msg);
            }
        }

        //
        // This is how the console app will log its message.
        //

        public void logToConsole(string msg)
        {
            Console.WriteLine(msg);
        }

        //
        // This is how the window form app will log its message.
        //

        public void logToOutputWindow(string msg)
        {
            System.Diagnostics.Debug.WriteLine(msg);
        }

        public void executeOnConsole()
        {
            initialize();

            //
            // when we start, the naked singles are not yet discovered
            // so we find the naked single here which cleans up the puzzle
            // and gets the puzzle into the state where most games are displayed to start
            //

            bool didSomething;
            didSomething = findNakedSingles();

            //
            // Register a callbackup with the fireLogEvent event.
            //

            fireLogEvent += logToConsole;

            display("Start of Puzzle:");

            solve();

            display("End of Puzzle:");

            //
            // Unregister a callbackup with the fireLogEvent event.
            //

            fireLogEvent -= logToConsole;
        }
    }
}
