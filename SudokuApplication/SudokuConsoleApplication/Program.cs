﻿using System;
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
                // "563700000002000947040100000030050209020000080409010050000004010254000600000006495"
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

                // http://www.palmsudoku.com tricky
                //  "700000093800003020030500806620008001000109000100600084306001050070400008910000007"
                //  "100000000059102000007800029005300604000060000304008500510004800000201950000000007"
                //  "563700000002000947040100000030050209020000080409010050000004010254000600000006495"
                //  "000730085000000006081040230128090070000000000070080642014070850500000000830051000"
                //  "013000060020800001900600700302008006060903020500100803001006009700004080080000610" // 6600
                //  "080403009000000137503700000070000002009806700800000060000009403495000000200507090" // 7500
                //  "500000080700500060060010700309700000200301005000005403002050010030009002090000006" // 7800
                // did not solve
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
                //  "100000058250100000079000400000092005500678034000051009014000600860900000900000081"
                // "100000058250100000079000400000092005500678034000051009014000600860900000900000081"

                // moderate: naked triple, simple coloring
                // triple at cell(4..6, 6)
                //  "007065009160000070400080006000200040600000003090003000800040007020000068900870300"
                // naked pair in column: (2 6) at cell(1,1) and cell (8,1); triple at cell(7,4..6)
                //  "050170304300002000409000000160300008000000000800006032000000405000900007705021080"
                // naked pair in column: (3 5) at cell(5,3) and cell(8,3)
                // "001007309300020400000008060107080604680204010904070508000006040700010800006005102"

                // http://mypuzzle.org/sudoku/ hard
                //  "045200007100059200000000000400600030700000008090004006000000000004830001500001960"
                // xwing
                //  "010346090300000006000105000120000047800407005004000900000020000700968003086030720"

                // Simple Sudoku
                // http://www.angusj.com/sudoku/
                // hard
                // 2 x naked triples
                // hidden pair ( 6 8 ) at cell(6,5) and cell(6,8), row 6
                // "5..8.......8..91...69..4...8.61....47...9...39....75.2...9..43...26..9.......3..7"
                // "........547.......85..42...64.58......79.41......73.96...85..34.......673........"
                // "46...1.....2.96....3.....68.......37...6.7...51.......84.....5....71.9.....3...24"
                // "1....89...5..9..32.9.7..6..83.9...2.....4...6.....53..5..1.94...6..5...84...8..1."
                // "...7..1...81..2..595...1..7.......73..3...8..54.......2..6...594..8..76...7..4..."
                // "73...46...94..28...2..........43.2.7.........3.2.78..........5...89..73...61...84"

                // triples
                // "4....961...56...79.1.42.3...51.6.................1.83...7.83.6.53...67...692....3"
                // "4....8.....753...8.9..6.41353....2.7.........7.6....81954.1..3.3...751.....9....5"

                // https://kjell.haxx.se/sudoku/
                //"....3......8..1.2.2....9......5..9.6.2......4..738...2..2.7..3.936..5...4..1....."
                //"4....5.83...........28...9.7.....469.6.41..2.85..6.3..9.......7..32.........9.8.5"
                //"4...........8..2.93.1....5.....4..1.5.....3..69.52.....5.79...8........72...3...."
                //"5....1...9.......8....6.7.........19...........3.7......68.......7...3.....9.5..2"

                // http://www.angusj.com/sudoku/ -- Advanced Puzzle Pack 2 -- puzzle001.ss to puzzle046.ss 
                // hidden pair ( 1 6 ) at cell(2,4) and cell(3,4), column 4
                // "2...578.37.....4...43.......7.54....32.876.49....13.5.......61...1.....44.736...2"
                // "7..42..6..435..2...1..3.59......4321....9....1643......71.6..3...9..361..2..41..9"
                // ".8..7..9...5..2...2.....4.8..7..61.5..2.3.6..1.82..9..3.1.....9...4..7...2..9..3."
                 "6.94..1...87.1......38.7....16.2...8..5...6..8...6.97....3.68......7.43...1..57.6"
                // "...9.61....3......2.9.5..48.5..1.3.7..8...4..1.7.3..2.73..2.5.4......7....43.5..."
                // "54..219..........5..695..1298...62.............27...6979..385..6..........856..71"
                // ".....7.21...6.8.......5..8693.7..4....6.4.5....4..9.6367..3.......1.6...14.9....."
                // "...8.6.4.5..7..31..43...8....5.....19..2.1..66.....9....2...63..69..2..8.5.1.7..."
                // "9.....3.1...21......8..6....659.82...4.....6...94.318....6..5......85...5.7.....3"
                // "65..4328..2.8.65.........648...24......198......76...878.........54.9.3..3458..26"
                // ".98..4..6.7..6....6.29.....9....6.21...4.8...56.1....9.....74.2....1..5.2..5..68."
                // "...1.....8...652.....47...8359.27.6..4.....5..2.54.7939...16.....428...7.....4..."
                // "5...9..6....5..1.7..4..1.9...14.765...........381.57...4.3..5..1.7..4....2..7...4"
                // ".32.9...5.....48....763...46....9.8...3...4...2.5....39...673....64.....2...8.15."
                // "..6..3.214.....6...98.7....6.31.........3.........54.6....4.16...1.....224.7..5.."
                // ".65...19.8..1.53..3.......575..2.......548.......1..245.......3..34.6..9.42...68."
                // "2.39.54......4.573...3.......7.....69..583..74.....8.......9...875.3......94.73.5"
                // "...5......469.853..8..32...82...41...94.2.86...18...43...24..5..721.568......3..."
                // "..2.4...13.5.69...47.1........7..13...14.27...63..8........1.78...68.9.38...7.5.."
                // "38..2.19...7...528..5..9.........8.4...4.7...5.2.........3..4..631...2...28.6..75"
                // ".3..2...67.....4..2.1.76....231...6.....5.....9...735....28.6.5..6.....19...1..4."
                // "4....268.8....1..2..9..6...29..8.4....4...3....6.4..18...7..1..6..9....4.416....7"
                // "2....5.986..8792.....2..7..3.54...8.4.......3.7...85.2..1..4.....8712..652.9....7"
                // ".46.9....3....8.7...8....32.2.1..6..8...3...9..1..9.5.41....8...8.6....4....7.12."
                // "2..59..43...........8327....76.....45.3.1.2.68.....93....1746...........69..53..1"
                // "..9..8.4.8.....3.7.3.9.2...49..7.2...7.....8...2.1..34...1.5.2.3.5.....8.6.3..5.."
                // ".....5.16174..2...65......9.....8.3...2.4.8...6.3.....4......68...5..27121.8....."
                // "..4.....9.2.5.......6.91.4......2.6.64..1..87.9.8......3.64.7.......7.5.1.....8.."
                // "..794...1....32..6...1...7...5.2..6996.....5882..9.4...4...1...6..27....5...697.."
                // ".2.1...3.45..32....3.7.54..1...78.....5...7.....25...1..24.7.1....58..94.8...1.6."
                // "..651..9.4.3..8.6..89..64......65.....41398.....24......56..12..3.4..5.9.4..526.."
                // ".974.6............1...5..9.4..9.2.5.2.18.59.7.5.3.1..6.4..1...5............6.931."
                // "8...65.9....2...57.2.8.....2.....7.5.6.....8.3.4.....9.....3.2.74...8....8.94...1"
                // ".54.7...8.2.8.5....7....35.7...286.....9.7.....241...9.98....1....3.4.8.5...8.96."
                // "......5..2..6....798..154..6..1..9.5..9.2.1..7.8..4..2..657..498....2..1..4......"
                // ".2.6..7.5.......1.9..17..8.5..2....8.1.9.7.5.7....6..2.8..94..3.5.......2.9..1.6."
                // ".53...1.44.....97.1..9....8.4.1........742........5.3.6....1..7.87.....12.1...64."
                // "5...6..7.431.7.......1..2....7..65...8..1..6...29..1....4..7.......5.723.5..2...9"
                // ".57.2..86..........1..8.47....4...595...9...373...6....78.5..9..........19..3.82."
                // "..8...1....5..2.647....1...5...4..9..39...42..7..3...6...8....921.4..6....6...5.."
                // ".5192..........32..2...3..59.658...1.386.295.2...198.47..2...1..13..........3467."
                // "1....4..2..3.7..4..8..6..7....7....654.....218....2....1..3..6..6..1.4..4..6....9"
                // "......1...1.....49..9.27.655..38.7.6.........2.7.64..317.83.9..95.....8...2......"
                // "7.9.3.8.....2.....4....8.6525...4...3..672..4...3...8794.8....3.....3.....3.1.7.8"
                // "4..2..89..96.8.2.1.......54...42.....5.9.1.8.....65...17.......6.2.4.51..39..2..8"
                // ".......5..5.9..3..3..754..8..8.4.6..17..2..45..5.7.2..5..867..4..9..5.8..6......."
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
        // find naked pair cells in row, column, or grid where both cells only have the exact two possibilities.
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

        public bool findHiddenPairsHorizontalNeighbors()
        {
            int NumChanges;
            bool[] PossiblePairs = new bool[9];
            int[,] PossiblePairLocations = new int[9, 2];

            NumChanges = 0;
            for (int row = 0; row<9; row++)
            {
                for (int k = 0; k < 9; k++)
                {
                    PossiblePairs[k] = false;
                    PossiblePairLocations[k, 0] = SudokuCell.NOTFOUND;
                    PossiblePairLocations[k, 1] = SudokuCell.NOTFOUND;
                    int numMatches = 0;
                    for (int column = 0; column < 9; column++)
                    { 
                        if ( !puzzle[row, column].IsSolved )
                        {
                            if ( puzzle[row, column].isPossible(k) )
                            {
                                if (numMatches < 2)
                                {
                                    PossiblePairLocations[k, numMatches] = column;
                                }
                                numMatches++;
                                PossiblePairs[k] = ( numMatches == 2 );
                            }
                        }
                    }
                }

                for (int p1 = 0; p1<9; p1++)
                {
                    if ( PossiblePairs[p1] )
                    {
                        for (int p2 = p1 + 1; p2 < 9; p2++)
                        {
                            if ( PossiblePairs[p2] )
                            {
                                bool shareSameTwoCells = (
                                        PossiblePairLocations[p1, 0] == PossiblePairLocations[p2, 0] &&
                                        PossiblePairLocations[p1, 1] == PossiblePairLocations[p2, 1]
                                    );
                                if ( shareSameTwoCells )
                                {
                                    int column1 = PossiblePairLocations[p1, 0];
                                    int column2 = PossiblePairLocations[p1, 1];
                                    int numChangesThisPair = 0;
                                    for (int k = 0; k<9; k++)
                                    {
                                        bool notInPair = ( k != p1 && k != p2 );
                                        if ( notInPair )
                                        {
                                            if ( puzzle[row, column1].isPossible(k) )
                                            {
                                                puzzle[row, column1].setPossible(k, false);
                                                numChangesThisPair++;
                                                NumChanges++;
                                            }
                                            if ( puzzle[row, column2].isPossible(k) )
                                            {
                                                puzzle[row, column2].setPossible(k, false);
                                                numChangesThisPair++;
                                                NumChanges++;
                                            }
                                        }
                                    }
                                    if ( numChangesThisPair > 0 )
                                    {
                                        Console.WriteLine("found a hidden pair ( {0} {1} ) at cell({2},{3}) and cell({4},{5}), row {6}",
                                            p1 + 1, p2 + 1, row + 1, column1 + 1, row + 1, column2 + 1, row + 1);
                                    }
                                    else
                                    {
                                        Console.WriteLine("xxx found a hidden pair ( {0} {1} ) at cell({2},{3}) and cell({4},{5}), row {6}",
                                            p1 + 1, p2 + 1, row + 1, column1 + 1, row + 1, column2 + 1, row + 1);
                                    }
                                }
                            }
                        }
                    }
                }
            }

            return ( NumChanges > 0 );
        }

        public bool findHiddenPairsVerticalNeighbors()
        {
            int NumChanges;
            bool[] PossiblePairs = new bool[9];
            int[,] PossiblePairLocations = new int[9, 2];

            NumChanges = 0;
            for (int column = 0; column < 9; column++)
            {
                for (int k = 0; k < 9; k++)
                {
                    PossiblePairs[k] = false;
                    PossiblePairLocations[k, 0] = SudokuCell.NOTFOUND;
                    PossiblePairLocations[k, 1] = SudokuCell.NOTFOUND;
                    int numMatches = 0;
                    for (int row = 0; row < 9; row++)
                    {
                        if ( !puzzle[row, column].IsSolved )
                        {
                            if ( puzzle[row, column].isPossible(k ))
                            {
                                if ( numMatches < 2 )
                                {
                                    PossiblePairLocations[k, numMatches] = row;
                                }
                                numMatches++;
                                PossiblePairs[k] = (numMatches == 2);
                            }
                        }
                    }
                }

                for (int p1 = 0; p1 < 9; p1++)
                {
                    if ( PossiblePairs[p1] )
                    {
                        for (int p2 = p1 + 1; p2 < 9; p2++)
                        {
                            if ( PossiblePairs[p2] )
                            {
                                bool shareSameTwoCells = (
                                      PossiblePairLocations[p1, 0] == PossiblePairLocations[p2, 0] &&
                                      PossiblePairLocations[p1, 1] == PossiblePairLocations[p2, 1]
                                    );
                                if ( shareSameTwoCells )
                                {
                                    int row1 = PossiblePairLocations[p1, 0];
                                    int row2 = PossiblePairLocations[p1, 1];
                                    int numChangesThisPair = 0;
                                    for (int k = 0; k < 9; k++)
                                    {
                                        bool notInPair = (k != p1 && k != p2);
                                        if (notInPair)
                                        {
                                            if ( puzzle[row1, column].isPossible(k) )
                                            {
                                                puzzle[row1, column].setPossible(k, false);
                                                numChangesThisPair++;
                                                NumChanges++;
                                            }
                                            if ( puzzle[row2, column].isPossible(k) )
                                            {
                                                puzzle[row2, column].setPossible(k, false);
                                                numChangesThisPair++;
                                                NumChanges++;
                                            }
                                        }
                                    }
                                    if ( numChangesThisPair > 0 )
                                    {
                                        Console.WriteLine("found a hidden pair ( {0} {1} ) at cell({2},{3}) and cell({4},{5}), column {6}",
                                            p1 + 1, p2 + 1, row1 + 1, column + 1, row2 + 1, column + 1, column + 1);
                                    }
                                    else
                                    {
                                        Console.WriteLine("xxx found a hidden pair ( {0} {1} ) at cell({2},{3}) and cell({4},{5}), column {6}",
                                            p1 + 1, p2 + 1, row1 + 1, column + 1, row2 + 1, column + 1, column + 1);
                                    }
                                }
                            }
                        }
                    }
                }
            }

            return ( NumChanges > 0 );
        }

        public bool findHiddenPairsGridNeighbors()
        {
            int NumChanges;
            bool[] PossiblePairs = new bool[9];
            int[,] PossiblePairRowLocations = new int[9, 2];
            int[,] PossiblePairColumnLocations = new int[9, 2];

            NumChanges = 0;

            for (int gridi = 0; gridi < 3; gridi++)
            {
                for (int gridj = 0; gridj < 3; gridj++)
                {
                    for (int k = 0; k <9; k++)
                    {
                        PossiblePairs[k] = false;
                        PossiblePairRowLocations[k, 0]    = SudokuCell.NOTFOUND;
                        PossiblePairRowLocations[k, 1]    = SudokuCell.NOTFOUND;
                        PossiblePairColumnLocations[k, 0] = SudokuCell.NOTFOUND;
                        PossiblePairColumnLocations[k, 1] = SudokuCell.NOTFOUND;
                        int numMatches = 0;

                        // traverse a single grid(i,j) and populate PossiblePairs[k]

                        int starti = gridi * 3;
                        int startj = gridj * 3;
                        for (int row = starti; row < starti+3; row++)
                        {
                            for (int column = startj; column < startj+3; column++)
                            {
                                if ( !puzzle[row, column].IsSolved )
                                {
                                    if ( puzzle[row, column].isPossible(k) )
                                    {
                                        if ( numMatches < 2 )
                                        {
                                            PossiblePairRowLocations[k, numMatches]    = row;
                                            PossiblePairColumnLocations[k, numMatches] = column;
                                        }
                                        numMatches++;
                                        PossiblePairs[k] = (numMatches == 2);
                                    }
                                }
                            }
                        }
                    }

                    for (int p1 = 0; p1 < 9; p1++)
                    {
                        if ( PossiblePairs[p1] )
                        {
                            for (int p2 = p1 + 1; p2 < 9; p2++)
                            {
                                if ( PossiblePairs[p2] )
                                {
                                    bool shareSameTwoCells = (
                                            PossiblePairRowLocations[p1, 0] == PossiblePairRowLocations[p2, 0] &&
                                            PossiblePairRowLocations[p1, 1] == PossiblePairRowLocations[p2, 1] &&
                                            PossiblePairColumnLocations[p1, 0] == PossiblePairColumnLocations[p2, 0] &&
                                            PossiblePairColumnLocations[p1, 1] == PossiblePairColumnLocations[p2, 1]
                                        );
                                    if  ( shareSameTwoCells )
                                    {
                                        int row1 = PossiblePairRowLocations[p1, 0];
                                        int row2 = PossiblePairRowLocations[p1, 1];
                                        int column1 = PossiblePairColumnLocations[p1, 0];
                                        int column2 = PossiblePairColumnLocations[p1, 1];
                                        int numChangesThisPair = 0;
                                        for (int k = 0; k < 9; k++)
                                        {
                                            bool notInPair = (k != p1 && k != p2);
                                            if ( notInPair )
                                            {
                                                if ( puzzle[row1, column1].isPossible(k) )
                                                {
                                                    puzzle[row1, column1].setPossible(k, false);
                                                    numChangesThisPair++;
                                                    NumChanges++;
                                                }
                                                if ( puzzle[row2, column2].isPossible(k) )
                                                {
                                                    puzzle[row2, column2].setPossible(k, false);
                                                    numChangesThisPair++;
                                                    NumChanges++;
                                                }
                                            }
                                        }
                                        if ( numChangesThisPair > 0 )
                                        {
                                            Console.WriteLine("found a hidden pair ( {0} {1} ) at grid({2},{3}) in cell({4},{5}) and cell({6},{7})",
                                                p1 + 1, p2 + 1, gridi + 1, gridj + 1, row1 + 1, column1 + 1, row2 + 1, column2 + 1);
                                        }
                                        else
                                        {
                                            Console.WriteLine("xxx found a hidden pair ( {0} {1} ) at grid({2},{3}) in cell({4},{5}) and cell({6},{7})",
                                                p1 + 1, p2 + 1, gridi + 1, gridj + 1, row1 + 1, column1 + 1, row2 + 1, column2 + 1);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            return (NumChanges > 0);
        }

        //
        // find hidden pair cells in row, column, or grid where both cells only have other possibilities.
        // from two hidden pair cells, remove possibilities from non-pairs
        //

        public bool findHiddenPairs()
        {
            bool foundPair = false;
            //display("before find hidden pairs:");
            foundPair |= findHiddenPairsHorizontalNeighbors();
            foundPair |= findHiddenPairsVerticalNeighbors();
            foundPair |= findHiddenPairsGridNeighbors();
            //display("after find hidden pairs:");
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
                if ( !didSomething )
                {
                    didSomething = findHiddenSingles();
                    if ( !didSomething )
                    {
                        didSomething = findNakedPairs();
                        if ( !didSomething )
                        {
                            didSomething = findHiddenPairs();
                            if ( !didSomething )
                            {
                                didSomething = findLockedCandidate();
                            }
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
