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
            return
              // hidden single in row, column, grid; naked pair in row;
              // type 1 and 2 locked candidate in row and column
               "563700000002000947040100000030050209020000080409010050000004010254000600000006495"
              // aka
              /*
                     "5637....." +
                     "..2...947" +
                     ".4.1....." +
                     ".3..5.2.9" +
                     ".2.....8." +
                     "4.9.1..5." +
                     ".....4.1." +
                     "254...6.." +
                     ".....6495";
               */

              // http://www.palmsudoku.com trivky
              //  "700000093800003020030500806620008001000109000100600084306001050070400008910000007"
              //  "100000000059102000007800029005300604000060000304008500510004800000201950000000007"
              //  "563700000002000947040100000030050209020000080409010050000004010254000600000006495"
              //  "000730085000000006081040230128090070000000000070080642014070850500000000830051000"
              //  "013000060020800001900600700302008006060903020500100803001006009700004080080000610" // 6600
              //  "080403009000000137503700000070000002009806700800000060000009403495000000200507090" // 7500
              //  "500000080700500060060010700309700000200301005000005403002050010030009002090000006" // 7800
              //  "208053700900000000000406008504810200100000007009024105600208000000000009007540302" // 9150
              //  "106000002000080090398005000580302000000090000000408023000200756070040000200000804" // 6850
              //  "019058000503201000700000005040900008000060000800004030900000006000406209000390750" // 6900
              //  "000070520083560000000002080002000004050603010900000200010700000000086740064010000" // 8150
              // found a naked pair in row, column and grid
              //  "000085400050003027000000008402007050608000704070300206500000000940500060001920000" // 6750
              //  "502714008060000000700000300001070000090802070000050400007000009000000060400193807" // 8750
              //  "009300000000080401710400060030000002008209100100000030080001075401050000000003600" // 7850

             // http://mypuzzle.org/sudoku/ moderate
             //  "300040598029100000000000004000002903000090000207400000700000000000005870856070002"
             // hidden triple (3,6,7) at cell(2, 7..9)
             // "300040598029100000000000004000002903000090000207400000700000000000005870856070002"

             // moderate: naked triple, simple coloring
             // triple at cell(4..6, 6)
             //  "007065009160000070400080006000200040600000003090003000800040007020000068900870300"
             // naked pair in column: (2 6) at cell(1,1) and cell (8,1); triple at cell(7,4..6)
             //  "050170304300002000409000000160300008000000000800006032000000405000900007705021080"
             // naked pair in column: (3 5) at cell(5,3) and cell(8,3)
             // "001007309300020400000008060107080604680204010904070508000006040700010800006005102"

             // http://mypuzzle.org/sudoku/ hard
             //  "045200007100059200000000000400600030700000008090004006000000000004830001500001960"

             ;
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

                    // when the input string does not have a solution for sudoku cell,
                    // value will be either '0' or '.' (how other programs write puzzles to stdout)
                    // normalize to '.' hre
                    if (cell == '0') cell = '.';

                    if ( Char.IsDigit(cell) )
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

        public bool findNakedPairsHorizontalNeighbors()
        {
            int numCandidatePairs;
            int[,] CandidatePairs = new int[9,4];
            int numNakedPairs;
            int[,] NakedPairs = new int[9,6];

            int numChanges = 0;
            for (int row = 0; row<9; row++)
            {
                numCandidatePairs = 0;
                numNakedPairs = 0;
                for (int column = 0; column<9; column++)
                {
                    if ( !puzzle[row, column].IsSolved )
                    {
                        int value1 = SudokuCell.NOTFOUND;
                        int value2 = SudokuCell.NOTFOUND;
                        int numPossibilities = 0;
                        for (int k =0; k<9; k++)
                        {
                            if ( puzzle[row, column].isPossible(k) )
                            {
                                numPossibilities++;
                                if (numPossibilities > 2) break;
                                if ( value1 == SudokuCell.NOTFOUND ) value1 = k;
                                else                                 value2 = k;
                            }
                        }
                        if ( numPossibilities == 2 )
                        {
                            CandidatePairs[numCandidatePairs, 0] = row;
                            CandidatePairs[numCandidatePairs, 1] = column;
                            CandidatePairs[numCandidatePairs, 2] = value1;
                            CandidatePairs[numCandidatePairs, 3] = value2;
                            numCandidatePairs++;
                        }
                    }
                }
                if ( numCandidatePairs > 0 )
                {
                    for (int pair = 0; pair<numCandidatePairs; pair++)
                    {
                        //Console.WriteLine("found candidate pair( {0} {1} ) at cell({2},{3})", 
                        //    CandidatePairs[pair, 2]+1, CandidatePairs[pair, 3]+1,
                        //    CandidatePairs[pair, 0]+1, CandidatePairs[pair, 1]+1);
                    }
                }
                if ( numCandidatePairs > 1 )
                {
                    // look for matching candidate pairs
                    for (int i = 0; i<numCandidatePairs; i++)
                    {
                        for (int j = i + 1; j<numCandidatePairs; j++)
                        {
                            bool foundMatch = ( CandidatePairs[i, 2] == CandidatePairs[j, 2] &&
                                                CandidatePairs[i, 3] == CandidatePairs[j, 3]);
                            if ( foundMatch )
                            {
                                // got a naked pair (p1,p2) at cell(i1,j1) and cell(i2,j2) where i1==i2
                                int i1 = CandidatePairs[i, 0];
                                int j1 = CandidatePairs[i, 1];
                                int i2 = CandidatePairs[j, 0];
                                int j2 = CandidatePairs[j, 1];
                                int p1 = CandidatePairs[i, 2];
                                int p2 = CandidatePairs[i, 3];

                                Console.WriteLine("found a naked pair( {0} {1} ) at cell({2},{3}) and cell({4},{5}) row {6}",
                                                  p1 + 1, p2 + 1, i1 + 1, j1 + 1, i2 + 1, j2 + 1, row + 1);

                                NakedPairs[numNakedPairs, 0] = i1;
                                NakedPairs[numNakedPairs, 1] = j1;
                                NakedPairs[numNakedPairs, 2] = i2;
                                NakedPairs[numNakedPairs, 3] = j2;
                                NakedPairs[numNakedPairs, 4] = p1;
                                NakedPairs[numNakedPairs, 5] = p2;
                                numNakedPairs++;
                            }
                        }
                    }
                }
                if ( numNakedPairs > 0 )
                {
                    for (int column = 0; column < 9; column++)
                    {
                        for (int pair = 0; pair < numNakedPairs; pair++)
                        {
                            // got a naked pair (p1,p2) at cell(i1,j1) and cell(i2,j2) where i1==i2
                            int i1 = NakedPairs[pair, 0];
                            int j1 = NakedPairs[pair, 1];
                            int i2 = NakedPairs[pair, 2];
                            int j2 = NakedPairs[pair, 3];
                            bool lookingAtPairCell = ( column == j1 || column == j2 );
                            if ( !lookingAtPairCell )
                            {
                                // pair (p1,p2) is no longer a possibility for this cell(row,column)
                                int p1 = NakedPairs[pair, 4];
                                int p2 = NakedPairs[pair, 5];
                                if ( puzzle[row, column].isPossible(p1) )
                                {
                                    numChanges++;
                                    puzzle[row, column].setPossible(p1, false);
                                }
                                if ( puzzle[row, column].isPossible(p2) )
                                {
                                    numChanges++;
                                    puzzle[row, column].setPossible(p2, false);
                                }
                            }
                        }
                    }
                }
            }

            return ( numChanges > 0 );
        }

        public bool findNakedPairsVerticalNeighbors()
        {
            int numCandidatePairs;
            int[,] CandidatePairs = new int[9, 4];
            int numNakedPairs;
            int[,] NakedPairs = new int[9, 6];

            int numChanges = 0;
            for (int column = 0; column < 9; column++)
            {
                numCandidatePairs = 0;
                numNakedPairs = 0;
                for (int row = 0; row < 9; row++)
                {
                    if ( !puzzle[row, column].IsSolved )
                    {
                        int value1 = SudokuCell.NOTFOUND;
                        int value2 = SudokuCell.NOTFOUND;
                        int numPossibilities = 0;
                        for (int k = 0; k < 9; k++)
                        {
                            if ( puzzle[row, column].isPossible(k) )
                            {
                                numPossibilities++;
                                if (numPossibilities > 2) break;
                                if (value1 == SudokuCell.NOTFOUND) value1 = k;
                                else value2 = k;
                            }
                        }
                        if ( numPossibilities == 2 )
                        {
                            CandidatePairs[numCandidatePairs, 0] = row;
                            CandidatePairs[numCandidatePairs, 1] = column;
                            CandidatePairs[numCandidatePairs, 2] = value1;
                            CandidatePairs[numCandidatePairs, 3] = value2;
                            numCandidatePairs++;
                        }
                    }
                }
                if ( numCandidatePairs > 0 )
                {
                    for (int pair = 0; pair < numCandidatePairs; pair++)
                    {
                        //Console.WriteLine("found candidate pair( {0} {1} ) at cell({2},{3})",
                        //    CandidatePairs[pair, 2] + 1, CandidatePairs[pair, 3] + 1,
                        //    CandidatePairs[pair, 0] + 1, CandidatePairs[pair, 1] + 1);
                    }
                }
                if ( numCandidatePairs > 1 )
                {
                    // look for matching candidate pairs
                    for (int i = 0; i < numCandidatePairs; i++)
                    {
                        for (int j = i + 1; j < numCandidatePairs; j++)
                        {
                            bool foundMatch = ( CandidatePairs[i, 2] == CandidatePairs[j, 2] &&
                                                CandidatePairs[i, 3] == CandidatePairs[j, 3] );
                            if ( foundMatch )
                            {
                                // got a naked pair (p1,p2) at cell(i1,j1) and cell(i2,j2) where i1==i2
                                int i1 = CandidatePairs[i, 0];
                                int j1 = CandidatePairs[i, 1];
                                int i2 = CandidatePairs[j, 0];
                                int j2 = CandidatePairs[j, 1];
                                int p1 = CandidatePairs[i, 2];
                                int p2 = CandidatePairs[i, 3];

                                Console.WriteLine("found a naked pair( {0} {1} ) at cell({2},{3}) and cell({4},{5}) column {6}",
                                                  p1 + 1, p2 + 1, i1 + 1, j1 + 1, i2 + 1, j2 + 1, column + 1);

                                NakedPairs[numNakedPairs, 0] = i1;
                                NakedPairs[numNakedPairs, 1] = j1;
                                NakedPairs[numNakedPairs, 2] = i2;
                                NakedPairs[numNakedPairs, 3] = j2;
                                NakedPairs[numNakedPairs, 4] = p1;
                                NakedPairs[numNakedPairs, 5] = p2;
                                numNakedPairs++;
                            }
                        }
                    }
                }
                if ( numNakedPairs > 0 )
                {
                    for (int row = 0; row < 9; row++)
                    {
                        for (int pair = 0; pair < numNakedPairs; pair++)
                        {
                            // got a naked pair (p1,p2) at cell(i1,j1) and cell(i2,j2) where i1==i2
                            int i1 = NakedPairs[pair, 0];
                            int j1 = NakedPairs[pair, 1];
                            int i2 = NakedPairs[pair, 2];
                            int j2 = NakedPairs[pair, 3];
                            bool lookingAtPairCell = (row == i1 || row == i2);
                            if (!lookingAtPairCell)
                            {
                                // pair (p1,p2) is no longer a possibility for this cell(row,column)
                                int p1 = NakedPairs[pair, 4];
                                int p2 = NakedPairs[pair, 5];
                                if (puzzle[row, column].isPossible(p1))
                                {
                                    numChanges++;
                                    puzzle[row, column].setPossible(p1, false);
                                }
                                if (puzzle[row, column].isPossible(p2))
                                {
                                    numChanges++;
                                    puzzle[row, column].setPossible(p2, false);
                                }
                            }
                        }
                    }
                }
            }

            return ( numChanges > 0 );
        }

        public bool findNakedPairsGridNeighbors()
        {
            int numCandidatePairs;
            int[,] CandidatePairs = new int[9, 4];
            int numNakedPairs;
            int[,] NakedPairs = new int[9, 6];

            int numChanges = 0;
            for (int gridi = 0; gridi < 3; gridi++)
            {
                for (int gridj = 0; gridj < 3; gridj++)
                {
                    int starti = gridi * 3;
                    int startj = gridj * 3;
                    numCandidatePairs = 0;
                    numNakedPairs = 0;
                    for (int row = starti; row < starti + 3; row++)
                    {
                        for (int column = startj; column < startj + 3; column++)
                        {
                            int numPossibilities = 0;
                            if ( !puzzle[row, column].IsSolved )
                            {
                                int value1 = SudokuCell.NOTFOUND;
                                int value2 = SudokuCell.NOTFOUND;
                                for (int k = 0; k < 9; k++)
                                {
                                    if ( puzzle[row, column].isPossible(k) )
                                    {
                                        numPossibilities++;
                                        if ( numPossibilities > 2 ) break;
                                        if ( value1 == SudokuCell.NOTFOUND ) value1 = k;
                                        else                                 value2 = k;
                                    }
                                }
                                if (numPossibilities == 2 )
                                {
                                    CandidatePairs[numCandidatePairs, 0] = row;
                                    CandidatePairs[numCandidatePairs, 1] = column;
                                    CandidatePairs[numCandidatePairs, 2] = value1;
                                    CandidatePairs[numCandidatePairs, 3] = value2;
                                    numCandidatePairs++;
                                }
                            }
                        }
                    }
                    if ( numCandidatePairs > 0 )
                    {
                        for (int pair = 0; pair < numCandidatePairs; pair++)
                        {
                            //Console.WriteLine("found candidate pair( {0} {1} ) at cell({2},{3}) in grid({4},{5})",
                            //                  CandidatePairs[pair, 2] + 1, CandidatePairs[pair, 3] + 1,
                            //                  CandidatePairs[pair, 0] + 1, CandidatePairs[pair, 1] + 1,
                            //                  gridi + 1, gridj + 1);
                        }
                    }
                    if ( numCandidatePairs > 1 )
                    {
                        // look for matching candidate pairs
                        for (int i = 0; i < numCandidatePairs; i++)
                        {
                            for (int j = i + 1; j < numCandidatePairs; j++)
                            {
                                bool foundMatch = ( CandidatePairs[i, 2] == CandidatePairs[j, 2] &&
                                                    CandidatePairs[i, 3] == CandidatePairs[j, 3] );
                                if ( foundMatch )
                                {
                                    // got a naked pair (p1,p2) at cell(i1,j1) and cell(i2,j2) where i1==i2
                                    int i1 = CandidatePairs[i, 0];
                                    int j1 = CandidatePairs[i, 1];
                                    int i2 = CandidatePairs[j, 0];
                                    int j2 = CandidatePairs[j, 1];
                                    int p1 = CandidatePairs[i, 2];
                                    int p2 = CandidatePairs[i, 3];

                                    Console.WriteLine("found a naked pair( {0} {1} ) at cell({2},{3}) and cell({4},{5}) grid({6},{7})",
                                                      p1 + 1, p2 + 1, i1 + 1, j1 + 1, i2 + 1, j2 + 1, gridi + 1, gridj + 1);

                                    NakedPairs[numNakedPairs, 0] = i1;
                                    NakedPairs[numNakedPairs, 1] = j1;
                                    NakedPairs[numNakedPairs, 2] = i2;
                                    NakedPairs[numNakedPairs, 3] = j2;
                                    NakedPairs[numNakedPairs, 4] = p1;
                                    NakedPairs[numNakedPairs, 5] = p2;
                                    numNakedPairs++;
                                }
                            }
                        }
                    }
                    if ( numNakedPairs > 0 )
                    {
                        for (int row = starti; row < starti + 3; row++)
                        {
                            for (int column = startj; column < startj + 3; column++)
                            {
                                for (int pair = 0; pair < numNakedPairs; pair++)
                                {
                                    // got a naked pair (p1,p2) at cell(i1,j1) and cell(i2,j2) where i1==i2
                                    int i1 = NakedPairs[pair, 0];
                                    int j1 = NakedPairs[pair, 1];
                                    int i2 = NakedPairs[pair, 2];
                                    int j2 = NakedPairs[pair, 3];
                                    bool lookingAtPairCell = ( (row == i1 && column == j1) || 
                                                               (row == i2 && column == j2) );
                                    if ( !lookingAtPairCell )
                                    {
                                        // pair (p1,p2) is no longer a possibility for this cell(row,column)
                                        int p1 = NakedPairs[pair, 4];
                                        int p2 = NakedPairs[pair, 5];
                                        if (puzzle[row, column].isPossible(p1))
                                        {
                                            numChanges++;
                                            puzzle[row, column].setPossible(p1, false);
                                        }
                                        if (puzzle[row, column].isPossible(p2))
                                        {
                                            numChanges++;
                                            puzzle[row, column].setPossible(p2, false);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            return ( numChanges > 0 );
        }

        //
        // find pair cells in row, column, or grid where both cells only have the exact two possibilities.
        // other cells in the row, column, or grid can then eliminate those two possibilities
        //

        public bool findNakedPairs()
        {
            bool foundPair = false;
            foundPair |= findNakedPairsHorizontalNeighbors();
            foundPair |= findNakedPairsVerticalNeighbors();
            foundPair |= findNakedPairsGridNeighbors();
            return foundPair;
        }

        public bool findLockedCandidateHorizontal()
        {
            int numChanges;
            bool[,,] Candidate = new bool[9,3,9];

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
                        if ( !puzzle[row, column].IsSolved )
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
                        if ( Candidate[starti, gridj, k] && !Candidate[starti + 1, gridj, k] && !Candidate[starti + 2, gridj, k] )
                        {
                            foundLockedCandidate = true;
                            lockedRow = starti;
                        }
                        if ( !Candidate[starti, gridj, k] && Candidate[starti + 1, gridj, k] && !Candidate[starti + 2, gridj, k] )
                        {
                            foundLockedCandidate = true;
                            lockedRow = starti+1;
                        }
                        if ( !Candidate[starti, gridj, k] && !Candidate[starti + 1, gridj, k] && Candidate[starti + 2, gridj, k] )
                        {
                            foundLockedCandidate = true;
                            lockedRow = starti+2;
                        }
                        if ( foundLockedCandidate )
                        {
                            // row includes Candidate[lockedRow, 0, k] + Candidate[lockedRow, 1, k] +  Candidate[lockedRow, 2, k]
                            bool foundPossibilitiesToRemove = (
                                  (gridj != 0 && Candidate[lockedRow, 0, k]) ||
                                  (gridj != 1 && Candidate[lockedRow, 1, k]) ||
                                  (gridj != 2 && Candidate[lockedRow, 2, k]) 
                                );
                            if ( foundPossibilitiesToRemove )
                            {
                                Console.WriteLine("found type 1 locked candidated {0} in row cells({1},{2}..{3}), exclude the rest of the row",
                                                  k + 1, lockedRow + 1, startj + 1, startj + 3);
                                for (int column = 0; column < 9; column++)
                                {
                                    int thisGridColumn = column / 3;
                                    bool gridToExclude = ( thisGridColumn == gridj );
                                    if ( !gridToExclude )
                                    {
                                        if ( puzzle[lockedRow, column].isPossible(k) )
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
                    if ( Candidate[row, 0, k] && !Candidate[row, 1, k] && !Candidate[row, 2, k] )
                    {
                        foundLockedCandidate = true;
                        startj = 0;
                    }
                    if ( !Candidate[row, 0, k] && Candidate[row, 1, k] && !Candidate[row, 2, k] )
                    {
                        foundLockedCandidate = true;
                        startj = 3;
                    }
                    if ( !Candidate[row, 0, k] && !Candidate[row, 1, k] && Candidate[row, 2, k] )
                    {
                        foundLockedCandidate = true;
                        startj = 6;
                    }
                    if ( foundLockedCandidate )
                    {
                        int gridj = startj / 3;
                        int starti = row / 3 * 3;
                        // grid includes Candidate[starti, gridj, k] + Candidate[starti+1, gridj, k] + Candidate[starti+2, gridj, k]
                        bool foundPossibilitiesToRemove = (
                              (starti   != row && Candidate[starti,   gridj, k]) ||
                              (starti+1 != row && Candidate[starti+1, gridj, k]) ||
                              (starti+2 != row && Candidate[starti+2, gridj, k])
                            );
                        if ( foundPossibilitiesToRemove )
                        {
                            Console.WriteLine("found type 2 locked candidated {0} in row cells({1},{2}..{3}), exclude the rest of the grid",
                                              k + 1, row + 1, startj + 1, startj + 3);

                            for (int i = starti; i < starti + 3; i++)
                            {
                                bool excludeLockedRow = (i == row);
                                if (!excludeLockedRow)
                                {
                                    for (int j = startj; j < startj + 3; j++)
                                    {
                                        if (puzzle[i, j].isPossible(k) )
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
            
            return ( numChanges > 0 );
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
                        if ( !puzzle[row, column].IsSolved )
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
                        if ( Candidate[startj, gridi, k] && !Candidate[startj + 1, gridi, k] && !Candidate[startj + 2, gridi, k] )
                        {
                            foundLockedCandidate = true;
                            lockedColumn = startj;
                        }
                        if ( !Candidate[startj, gridi, k] && Candidate[startj + 1, gridi, k] && !Candidate[startj + 2, gridi, k] )
                        {
                            foundLockedCandidate = true;
                            lockedColumn = startj + 1;
                        }
                        if ( !Candidate[startj, gridi, k] && !Candidate[startj + 1, gridi, k] && Candidate[startj + 2, gridi, k] )
                        {
                            foundLockedCandidate = true;
                            lockedColumn = startj + 2;
                        }
                        if ( foundLockedCandidate )
                        {
                            // row includes Candidate[lockedRow, 0, k] + Candidate[lockedRow, 1, k] +  Candidate[lockedRow, 2, k]
                            bool foundPossibilitiesToRemove = (
                                  (gridi != 0 && Candidate[lockedColumn, 0, k]) ||
                                  (gridi != 1 && Candidate[lockedColumn, 1, k]) ||
                                  (gridi != 2 && Candidate[lockedColumn, 2, k])
                                );
                            if ( foundPossibilitiesToRemove )
                            {
                                Console.WriteLine("found type 1 locked candidated {0} in column cells({1}..{2},{3}), exclude the rest of the column",
                                                  k + 1, starti + 1, starti + 3, lockedColumn + 1);
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
                    if ( Candidate[column, 0, k] && !Candidate[column, 1, k] && !Candidate[column, 2, k] )
                    {
                        foundLockedCandidate = true;
                        starti = 0;
                    }
                    if ( !Candidate[column, 0, k] && Candidate[column, 1, k] && !Candidate[column, 2, k] )
                    {
                        foundLockedCandidate = true;
                        starti = 3;
                    }
                    if ( !Candidate[column, 0, k] && !Candidate[column, 1, k] && Candidate[column, 2, k] )
                    {
                        foundLockedCandidate = true;
                        starti = 6;
                    }
                    if ( foundLockedCandidate)
                    {
                        int gridi = starti / 3;
                        int startj = column / 3 * 3;
                        // grid includes Candidate[starti, gridj, k] + Candidate[starti+1, gridj, k] + Candidate[starti+2, gridj, k]
                        bool foundPossibilitiesToRemove = (
                              (startj     != column && Candidate[startj,     gridi, k]) ||
                              (startj + 1 != column && Candidate[startj + 1, gridi, k]) ||
                              (startj + 2 != column && Candidate[startj + 2, gridi, k])
                            );
                        if ( foundPossibilitiesToRemove )
                        {
                            Console.WriteLine("found type 2 locked candidated {0} in column cells({1}..{2},{3}), exclude the rest of the grid",
                                              k + 1, starti + 1, starti + 3, column + 1);

                            for (int j = startj; j < startj + 3; j++)
                            {
                                bool excludeLockedColumn = ( j == column );
                                if ( !excludeLockedColumn)
                                {
                                    for (int i = starti; i < starti + 3; i++)
                                    {
                                        if ( puzzle[i, j].isPossible(k) )
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

            return ( numChanges > 0 );
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
                        didSomething = findNakedPairs();
                        if (!didSomething)
                        {
                            didSomething |= findLockedCandidate();
                        }
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
