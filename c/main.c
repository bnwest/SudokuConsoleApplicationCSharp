#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>

// http://en.wikipedia.org/wiki/Quadtree

struct SudokuCell {
    int iFound;
    bool vPossibles[9];
};
typedef struct SudokuCell SudokuCell;

struct SudokuGrid {
    SudokuCell vCells[3][3];
};
typedef struct SudokuGrid SudokuGrid;

struct Sudoku {
    SudokuGrid vGrids[3][3];
};

#define NOTFOUND (-1)

// or

SudokuCell Sudoku[9][9];

// Given call Sudoku[i][j]
// horizontal neighbors:  Sudoku[i][*]
// vertical neighbors  :  Sudoku[*][j]
// grid neighbors      :
//   start_i = i / 3 * 3
//   statr_j = i / 3 * 3
//   Sudoku[start_i  ][start_j  ] is top corner
//   Sudoku[start_i+2][start_j+2] is bottom corner

char *strInput = 
    // http://www.sudokusolver.co.uk/lsolve
//  "_3___1___+__6____5_+5_____983+_8___63_2+____5____+9_38___6_+714_____9+_2____8__+___4___3_"
    
    // http://magictour.free.fr/msk_009
//  "............942.8.16.....29........89.6.....14..25......4.......2...8.9..5....7.."
//  ".5247.....6............8.1.4.......97..95.....2..4..3....8...9......37.6....91..."
//  ".9.........1..6....6..8..7.3......1.....39.......5...217.4...28.....3....86....57"
//  "5...68..........6..42.5.......8..9....1....4.9.3...62.7....1..9..42....3.8......."
//  ".7..21..4....3....6.1.....2.......6...86..7.319.....4..1....2.842.9.............."
//  "........1..7.5.3.9..48...2...........3...57....942.........3.....1...4.7.6.278..."
//  ".....6..3..9.4...532......8....1......175.6.92......8.....6.......8...4.47....2.."
//  ".2............48...54.18.3.7....1..4....86.5.......6........1......2...923.4....5"
//  "..9.43..........3.41..7.............8..5...6..4...6..2.......1...4.98..67..6..52."
//  ".........4.6.7..9..5..382.........3.9..........426.....7...3..2..16..8...85...7.."
//  "...6.4...........3.1...26....2......6...9..158.4.....6.....7...976.5.......2.31.."
//  "....4.....5......9..3.784....1......62..........5.38......2......64..7.34.51...2."
//  "...5....3...82...13....179.17.............3..6..712.4..4..6.....9........6..5.2.."
//  ".8.4.....13...............84....1...5.7..2..3...9..1......2.78.2....6.3..76..3..9"
//  ".5.........6..5.91..9...38.4.......8....38..2.73..........1....28.47.5..6......7."
//  ".1..6.9....9..5....3.....76..1.3...272.....4...8........73....93.5..76.........2."
//  "..........7...62.81......54..3.5.......3....22..8....69......8.3...7......7.254.1"
//  "..2.46.....4.8...5.7..3...9.....2...3.57.....7.....4....6....93....54.78........."

    // http://www.palmsudoku.com medium
//  "700000003130080050004200000020040000390010025000050090000001900040020018800000007"
//  "307900000005007000106080000208050007040030020500090608000070901000600300000005702"
//  "200410007075003000000000600000200470060307080093008000008000000000800940100026008"
//  "080050000009607058502014960600000002003000400900000006094370605230506800000040030"
//  "000042000600100904004800703026000008007000400900000310409006100502004007000580000"
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
//  "000085400050003027000000008402007050608000704070300206500000000940500060001920000" // 6750
//  "502714008060000000700000300001070000090802070000050400007000009000000060400193807" // 8750
  "009300000000080401710400060030000002008209100100000030080001075401050000000003600" // 7850


    // http://mypuzzle.org/sudoku/ simple
//  "106009082350408009008167453010500038000093100060700024001632895620805007805004061"
    // http://mypuzzle.org/sudoku/ moderate
//  "300040598029100000000000004000002903000090000207400000700000000000005870856070002"
// "300040598029100000000000004000002903000090000207400000700000000000005870856070002"

    // moderate: naked triple, simple coloring
//  "007065009160000070400080006000200040600000003090003000800040007020000068900870300"
//  "050170304300002000409000000160300008000000000800006032000000405000900007705021080"
//  "001007309300020400000008060107080604680204010904070508000006040700010800006005102"

    // http://mypuzzle.org/sudoku/ hard
//  "045200007100059200000000000400600030700000008090004006000000000004830001500001960"

// Simple Sudoku normal
//  ".......1.3...64..5..4..98..5.....7.4...938...6.1.....3..71..5..1..47...2.4......."
//  ".21......9.85..7.........6...79.8....59.3.82....1.26...7.........3..54.8......35."
//  ".7..4.6..96.8........6...987..5...1...9.8.5...2...7..385...1........8.61..3.2..4."
//  "95...7.3..26..........3...42.9..5.....74.19.....3..4.67...1..........86..8.2...75"
//  ".8.1..5.94.2.6........5.....362....47.......52....561.....9........7.3.19.4..8.6."


    // http://mypuzzle.org/sudoku/ too hard
//  "200000080106300005400105070000800040003000900050006000090602001500001704040000002"
//  "130249085000106000400070003020000090004000500000813000500000004003000800806354902"
//  "210030098308090205000000000400080006000951000890000051000000000980607014006509800"
//  "230400009108070000000500070004006900600000008005800300020004000000010206900003015"
//  "006239800300408006000000000402050309073802140005000700000000000030601020708020904"



    // TOO HARD
//  ".....7....9...1.......45..6....2.....36...41.5.....8.9........4....18....815...32"
//  ".94...13..............76..2.8..1.....32.........2...6.....5.4.......8..7..63.4..8"
//  ".....5....2...4.1..3..8..2......84..8..6......9..1.7.5..6......95...3.6...3.....1"
//  ".....8..3.16.2.9.7.3...46...........9.5...2...2.13...9..3....2..7...5.........4.."
//  "..4.2..3....8.9.........7...5..37..8........5.49.6..1.5.........68........7.4.9.1"
//  ".....8.2......693..98.7...1...........921....7......9624..9.......3..18.........3"
//  "..3....4.4..2.........9..26....7.....1.9.2...26......85....7.......6.8.33......69"
//  "........3..5..2.14....8..6..........946.......3...42.6...7.........3.68..7.291..."
//  ".2...7..5.........6...95..1.7...413.......2....1.5...67...1.8...8..7.......2...49"
//  "3......4...2..8....912...3...5.1..8..64.9.........5..618.7............65.7.9....."
//  "....4....1..9..64..3....8....7.........1.859.......3...52..1....1.7.3...39..5...4"
//  "4....9.....541...3........7.......2..31.7...89.6..3.......9....1..6...8...75...46"
//  "53..97..........7.....1..5......13....4..2...1.98..2.4........5.7....92..91.5...."
//  "...3.56....68..3...4.............8.5.5....412...9.......3.9........8..6..196.45.."


/* HARD : needs Y-Wing
123456789       123456789       123456789

.....7...       .5..87..1       653287941
.9...1...       .9..31..8       794631258
....45..6       1.8.453.6       128945376
....2....       819724..3       819724563
.36...41.       236859417       236859417
5.....8.9       547163829       547163829
........4       .653.2184       965372184
....18...       ...418..5       372418695
.815...32       4815.6.32       481596732

.....7....9...1.......45..6....2.....36...41.5.....8.9........4....18....815...32
.5..87..1.9..31..81.8.453.6819724..3236859417547163829.653.2184...418..54815.6.32
653287941794631258128945376819724563236859417547163829965372184372418695481596732
 */
;

void 
convertInput(char *strInput)
{
    char strLine[16];
    char *strStart;
    char strCell[2];
    int i, j, k, iCell, incr;
    
    incr = ( strInput[9] == '+' ? 10 : 9 );

    strStart = strInput;
    for (i=0; i<9; i++) {
        strncpy(strLine, strStart, 9);
        for (j=0; j<9; j++) {
            strCell[0] = strLine[j];
            strCell[1] = '\0';
            if ( '1' <= strCell[0] && strCell[0] <= '9' ) {
                iCell = atoi(strCell) - 1;
                Sudoku[i][j].iFound = NOTFOUND;
                for (k=0; k<9; k++) Sudoku[i][j].vPossibles[k] = false;
                Sudoku[i][j].vPossibles[iCell] = true;
            }
            else {
                Sudoku[i][j].iFound = NOTFOUND;
                for (k=0; k<9; k++) Sudoku[i][j].vPossibles[k] = true;
            }
        }
        strStart += incr;
    }
}

void 
writeOutput() 
{
//    "+-------+ +-------+ +-------+ +-------+ +-------+ +-------+ +-------+ +-------+ +-------+\n"
//    "| 1 2 3 | | 1 2 3 | | 1 2 3 | | 1 2 3 | | 1 2 3 | | 1 2 3 | | 1 2 3 | | 1 2 3 | | 1 2 3 |\n"
//    "| 4 5 6 | | 4 5 6 | | 4 5 6 | | 4 5 6 | | 4 5 6 | | 4 5 6 | | 4 5 6 | | 4 5 6 | | 4 5 6 |\n"
//    "| 7 8 9 | | 7 8 9 | | 7 8 9 | | 7 8 9 | | 7 8 9 | | 7 8 9 | | 7 8 9 | | 7 8 9 | | 7 8 9 |\n"
//    "+-------+ +-------+ +-------+ +-------+ +-------+ +-------+ +-------+ +-------+ +-------+\n"
    int i, j, k;
    for (i=0; i<9; i++)
    {
        printf("+-------+ +-------+ +-------+  +-------+ +-------+ +-------+  +-------+ +-------+ +-------+\n");
        for (k=0; k<9; k+=3) {
            for (j=0; j<9; j++) {            
                printf("| %c %c %c | ", 
                       Sudoku[i][j].vPossibles[k]   ? 0x30+k+1 : ' ',	
                       Sudoku[i][j].vPossibles[k+1] ? 0x30+k+2 : ' ',	
                       Sudoku[i][j].vPossibles[k+2] ? 0x30+k+3 : ' '
                );
                if ( (j+1) % 3 == 0 ) {
                    printf(" ");
                }
            }
            printf("\n");
        }
        printf("+-------+ +-------+ +-------+  +-------+ +-------+ +-------+  +-------+ +-------+ +-------+\n");
        if ( (i+1) % 3 == 0 ) {
            printf("\n");
        }
    }
}

bool
removeHorizontalNeighbors(int i, int j, int k)
{
    int column;
    bool foundRemoval;
    
    foundRemoval = false;
    for (column=0; column<9; column++) {
        if ( column != j && Sudoku[i][column].vPossibles[k] ) {
            foundRemoval = true;
            Sudoku[i][column].vPossibles[k] = false;
            printf("Remove %d from cell (%d,%d) horizontal neighbor\n", k+1, i+1, column+1);

        }
    }
    
    return foundRemoval;
}

bool
removeVerticalNeighbors(int i, int j, int k)
{
    int row;
    bool foundRemoval;
    
    foundRemoval = false;
    for (row=0; row<9; row++) {
        if ( row != i && Sudoku[row][j].vPossibles[k] ) {
            foundRemoval = true;
            Sudoku[row][j].vPossibles[k] = false;
            printf("Remove %d from cell (%d,%d) vertical neighbor\n", k+1, row+1, j+1);
            
        }
    }
    
    return foundRemoval;
}

bool
removeGridNeighbors(int i, int j, int k)
{
    int starti, startj, ii, jj;
    bool foundRemoval;
    
    foundRemoval = false;
    starti = i / 3 * 3;
    startj = j / 3 * 3;
    for (ii=starti; ii<starti+3; ii++) {
        for (jj=startj; jj<startj+3; jj++) {
            if ( ii != i && 
                 jj != j && 
                 Sudoku[ii][jj].vPossibles[k] ) {
                foundRemoval = true;
                Sudoku[ii][jj].vPossibles[k] = false;
                printf("Remove %d from cell (%d,%d) grid neighbor\n", k+1, ii+1, jj+1);
            }
        }
    }
    
    return foundRemoval;
}

bool
removeNeighbors(int i, int j, int k)
{
    bool foundRemoval;
    foundRemoval |= removeHorizontalNeighbors(i, j, k);
    foundRemoval |= removeVerticalNeighbors  (i, j, k);
    foundRemoval |= removeGridNeighbors      (i, j, k);
    return foundRemoval;
}

bool
findSingles()
{
    int i, j, k;
    int iNumPossibilities;
    bool foundSingle;
    bool foundSingles;
    bool foundRemoval;
    bool didSomething;
    int iFound;
    int iLastFound;
    int iValue;
    int Singles[9][9];
    int tobeRemovd[9][9][9];
    
    didSomething = false;
    
    foundSingles = false;
    for (i=0; i<9; i++) {
        for (j=0; j<9; j++) {
            if ( Sudoku[i][j].iFound == NOTFOUND ) {
                iNumPossibilities = 0;
                iFound = NOTFOUND;
                for (k=0; k<9; k++) {
                    if ( Sudoku[i][j].vPossibles[k] ) {
                        iLastFound = k;
                        iNumPossibilities++;
                        if ( iNumPossibilities > 1 ) break;
                    }
                }
                foundSingle = ( iNumPossibilities == 1 );
                if ( foundSingle ) {
                    foundSingles = true;
                    Singles[i][j] = iLastFound;
                    printf("found single %d at grid(%d,%d)\n", iLastFound+1, i+1, j+1);
                }
                else {
                    Singles[i][j] = NOTFOUND;
                }

            }
            else {
                Singles[i][j] = NOTFOUND;
            }
        }
    }
    
    foundRemoval = false;
    if ( foundSingles ) {
        for (i=0; i<9; i++) 
            for (j=0; j<9; j++)
                for (k=0; k<9; k++) tobeRemovd[i][j][k] = false;
        
        for (i=0; i<9; i++) {
            for (j=0; j<9; j++) {
                foundSingle = ( Singles[i][j] != NOTFOUND );
                if ( foundSingle ) {
                    iValue = Singles[i][j];
                    // mark cell as a single
                    Sudoku[i][j].iFound = iValue;
                    foundRemoval = removeNeighbors(i, j, iValue);
                }
            }
        }
        didSomething = foundRemoval;
    }
    
    return didSomething;
}

bool
findHiddenSingle() 
{
    int i, j, k, gridi, gridj, starti, startj;
    int iNumPossibilities;
    int iLastFound, iLastFoundI, iLastFoundJ;
    bool isSingleAlready;
    bool foundHiddenSingle;
    bool foundRemoval;
    int HiddenSingle[9][9];
    bool didSomething;
    
    didSomething = false;
    
    foundHiddenSingle = false;
    for (i=0; i<9; i++) {
        for (j=0; j<9; j++) {
            for (k=0; k<9; k++) {
                HiddenSingle[i][j] = NOTFOUND;
            }
        }
    }
    
    // find hidden single in horizontal neighbor
    for (i=0; i<9; i++) {
        // look for k in each ofthe row's cells
        for (k=0; k<9; k++) {
            iNumPossibilities = 0;
            iLastFound = NOTFOUND;
            for (j=0; j<9; j++) {
                if ( Sudoku[i][j].vPossibles[k] ) {
                    iNumPossibilities++;
                    iLastFound = j;
                    if ( iNumPossibilities > 1) break;
                }
            }
            if ( iNumPossibilities == 1 ) {
                isSingleAlready = ( Sudoku[i][iLastFound].iFound != NOTFOUND );
                if ( ! isSingleAlready ) {
                    HiddenSingle[i][iLastFound] = k;
                    foundHiddenSingle = true;
                    printf("found hidden single %d horizontally at grid(%d,%d)\n", k+1, i+1, iLastFound+1);
                }
            }
        }
    }

    // find hidden single in vertical neighbor
    for (j=0; j<9; j++) {
        // look for k in each ofthe column's cells
        for (k=0; k<9; k++) {
            iNumPossibilities = 0;
            iLastFound = NOTFOUND;
            for (i=0; i<9; i++) {
                if ( Sudoku[i][j].vPossibles[k] ) {
                    iNumPossibilities++;
                    iLastFound = i;
                    if ( iNumPossibilities > 1) break;
                }
            }
            if ( iNumPossibilities == 1 ) {
                isSingleAlready = ( Sudoku[iLastFound][j].iFound != NOTFOUND );
                if ( ! isSingleAlready ) {
                    if ( HiddenSingle[iLastFound][j] == NOTFOUND ) {
                        HiddenSingle[iLastFound][j] = k;
                        foundHiddenSingle = true;
                        printf("found hidden single %d vertically at grid(%d,%d)\n", k+1, iLastFound+1, j+1);
                    }
                }
            }
        }
    }
    
    // find hidden single in grid neighbor
    for (gridi=0; gridi<3; gridi++) {
        for (gridj=0; gridj<3; gridj++) {
            starti = gridi * 3;
            startj = gridj * 3;
            for (k=0; k<9; k++) {
                iLastFoundI = NOTFOUND;
                iLastFoundJ = NOTFOUND;
                iNumPossibilities = 0;
                for (i=starti; i<starti+3; i++) {
                    for (j=startj; j<startj+3; j++) {
                        if ( Sudoku[i][j].vPossibles[k] ) {
                            iNumPossibilities++;
                            iLastFoundI = i;
                            iLastFoundJ = j;
                            if ( iNumPossibilities > 1) break;
                        }
                    }
                    if ( iNumPossibilities > 1) break;
                }
                if ( iNumPossibilities == 1) {
                    isSingleAlready = ( Sudoku[iLastFoundI][iLastFoundJ].iFound != NOTFOUND );
                    if ( ! isSingleAlready ) {
                        if ( HiddenSingle[iLastFoundI][iLastFoundJ] == NOTFOUND ) {
                            HiddenSingle[iLastFoundI][iLastFoundJ] = k;
                            foundHiddenSingle = true;
                            printf("found hidden single %d within the grid at grid(%d,%d)\n", k+1, iLastFoundI+1, iLastFoundJ+1);
                        }
                    }
                }
            }
        }
    }
    
    foundRemoval = false;
    if ( foundHiddenSingle ) {
        for (i=0; i<9; i++) {
            for (j=0; j<9; j++) {
                if ( HiddenSingle[i][j] != NOTFOUND ) {
                    for (k=0; k<9; k++) {
                        if ( k != HiddenSingle[i][j] ) {
                            Sudoku[i][j].vPossibles[k] = false;
                        }
                    }
                    Sudoku[i][j].iFound = HiddenSingle[i][j];
                    foundRemoval = removeNeighbors(i, j, HiddenSingle[i][j]);
                }
            }
        }
        didSomething = foundRemoval;
    }
    return didSomething;
}

bool
findNakedPairsHorizontalNeighbors()
{
    int i, j, k, a, b, ii, jj, hp, j1, j2;
    int iNumPossibles;
    bool foundMatch;
    int iNumChanges;
    
    int iNumPairs;
    int Pairs[9][4];
    int iNumHiddenPairs;
    int HiddenPairs[9][6];

    iNumChanges = 0;
    for (i=0; i<9; i++) {
        // for each row
        iNumPairs = 0;
        iNumHiddenPairs = 0;
        for (j=0; j<9; j++) {
            // look at only the unsolved cells ...
            iNumPossibles = 0;
            if ( Sudoku[i][j].iFound == NOTFOUND ) {
                a = NOTFOUND;
                b = NOTFOUND;
                for (k=0; k<9; k++) {
                    if ( Sudoku[i][j].vPossibles[k] ) {
                        iNumPossibles++;
                        if ( a == NOTFOUND ) a = k;
                        else                 b = k;
                    }
                }
                if ( iNumPossibles == 2 ) {
                    Pairs[iNumPairs][0] = i;
                    Pairs[iNumPairs][1] = j;
                    Pairs[iNumPairs][2] = a;
                    Pairs[iNumPairs][3] = b;
                    iNumPairs++;
                }
            }
        }
        if ( iNumPairs > 0 ) {
            for (k=0; k<iNumPairs; k++) {
                printf("found pair( %d %d ) at cell(%d,%d)\n",
                       Pairs[k][2], Pairs[k][3],
                       Pairs[k][0], Pairs[k][1]);
            }
        }
        if ( iNumPairs > 1 ) {
            for (ii=0; ii<iNumPairs-1; ii++) {
                for (jj=ii+1; jj<iNumPairs; jj++) {
                    foundMatch = ( Pairs[ii][2] == Pairs[jj][2] &&
                                   Pairs[ii][3] == Pairs[jj][3] );
                    if ( foundMatch ) {
                        printf("found a hidden pair( %d %d ) at cell(%d,%d) and cell(%d,%d) row %d\n",
                               Pairs[ii][2]+1, Pairs[ii][3]+1,
                               Pairs[ii][0]+1, Pairs[ii][1]+1,
                               Pairs[jj][0]+1, Pairs[jj][1]+1,
                               i+1);
                        a = Pairs[ii][2];
                        b = Pairs[ii][3];
                        HiddenPairs[iNumHiddenPairs][0] = Pairs[ii][0];
                        HiddenPairs[iNumHiddenPairs][1] = Pairs[ii][1];
                        HiddenPairs[iNumHiddenPairs][2] = Pairs[jj][0];
                        HiddenPairs[iNumHiddenPairs][3] = Pairs[jj][1];
                        HiddenPairs[iNumHiddenPairs][4] = Pairs[ii][2];
                        HiddenPairs[iNumHiddenPairs][5] = Pairs[ii][3];
                        iNumHiddenPairs++;
                    }
                }
            }
        }
        if ( iNumHiddenPairs > 0 ) {
            for (j=0; j<9; j++) {
                // for each hidedn pair
                //    if (i1,j1) and (i2,j2) then skip
                //    for pair (a,b)
                //        if vPossible[a] then vPossible[a] = false
                //        if vPossible[a] then vPossible[a] = false
                for (hp=0; hp<iNumHiddenPairs; hp++) {
                    j1 = HiddenPairs[hp][1];
                    j2 = HiddenPairs[hp][3];
                    if ( j == j1 || j == j2 ) continue;
                    a = HiddenPairs[hp][4];
                    b = HiddenPairs[hp][5];
                    if ( Sudoku[i][j].vPossibles[a] ) {
                        iNumChanges++;
                        Sudoku[i][j].vPossibles[a] = false;
                        printf("remove %d from cell(%d,%d) horizontal neighbor\n", a+1, i+1, j+1);
                    }
                    if ( Sudoku[i][j].vPossibles[b] ) {
                        iNumChanges++;
                        Sudoku[i][j].vPossibles[b] = false;
                        printf("remove %d from cell(%d,%d) horizontal neighbor\n", b+1, i+1, j+1);
                    }
                }
            }
        }
    }
    
    return ( iNumChanges > 0 );
}

bool
findNakedPairsVerticalNeighbors()
{
    int i, j, k, a, b, ii, jj, hp, i1, i2;
    int iNumPossibles;
    bool foundMatch;
    int iNumChanges;
    
    int iNumPairs;
    int Pairs[9][4];
    int iNumHiddenPairs;
    int HiddenPairs[9][6];
    
    iNumChanges = 0;
    for (j=0; j<9; j++) {
        // for each column
        iNumPairs = 0;
        iNumHiddenPairs = 0;
        for (i=0; i<9; i++) {
            // look at only the unsolved cells ...
            iNumPossibles = 0;
            if ( Sudoku[i][j].iFound == NOTFOUND ) {
                a = NOTFOUND;
                b = NOTFOUND;
                for (k=0; k<9; k++) {
                    if ( Sudoku[i][j].vPossibles[k] ) {
                        iNumPossibles++;
                        if ( a == NOTFOUND ) a = k;
                        else                 b = k;
                    }
                }
                if ( iNumPossibles == 2 ) {
                    Pairs[iNumPairs][0] = i;
                    Pairs[iNumPairs][1] = j;
                    Pairs[iNumPairs][2] = a;
                    Pairs[iNumPairs][3] = b;
                    iNumPairs++;
                }
            }
        }
        if ( iNumPairs > 0 ) {
            for (k=0; k<iNumPairs; k++) {
                printf("found pair( %d %d ) at cell(%d,%d)\n",
                       Pairs[k][2], Pairs[k][3],
                       Pairs[k][0], Pairs[k][1]);
            }
        }
        if ( iNumPairs > 1 ) {
            for (ii=0; ii<iNumPairs-1; ii++) {
                for (jj=ii+1; jj<iNumPairs; jj++) {
                    foundMatch = ( Pairs[ii][2] == Pairs[jj][2] &&
                                   Pairs[ii][3] == Pairs[jj][3] );
                    if ( foundMatch ) {
                        printf("found a hidden pair( %d %d ) at cell(%d,%d) and cell(%d,%d) column %d\n",
                               Pairs[ii][2]+1, Pairs[ii][3]+1,
                               Pairs[ii][0]+1, Pairs[ii][1]+1,
                               Pairs[jj][0]+1, Pairs[jj][1]+1,
                               j+1);
                        a = Pairs[ii][2];
                        b = Pairs[ii][3];
                        HiddenPairs[iNumHiddenPairs][0] = Pairs[ii][0];
                        HiddenPairs[iNumHiddenPairs][1] = Pairs[ii][1];
                        HiddenPairs[iNumHiddenPairs][2] = Pairs[jj][0];
                        HiddenPairs[iNumHiddenPairs][3] = Pairs[jj][1];
                        HiddenPairs[iNumHiddenPairs][4] = Pairs[ii][2];
                        HiddenPairs[iNumHiddenPairs][5] = Pairs[ii][3];
                        iNumHiddenPairs++;
                    }
                }
            }
        }
        if ( iNumHiddenPairs > 0 ) {
            for (i=0; i<9; i++) {
                // for each hidedn pair
                //    if (i1,j1) and (i2,j2) then skip
                //    for pair (a,b)
                //        if vPossible[a] then vPossible[a] = false
                //        if vPossible[a] then vPossible[a] = false
                for (hp=0; hp<iNumHiddenPairs; hp++) {
                    i1 = HiddenPairs[hp][0];
                    i2 = HiddenPairs[hp][2];
                    if ( i == i1 || i == i2 ) continue;
                    a = HiddenPairs[hp][4];
                    b = HiddenPairs[hp][5];
                    if ( Sudoku[i][j].vPossibles[a] ) {
                        iNumChanges++;
                        Sudoku[i][j].vPossibles[a] = false;
                        printf("remove %d from cell(%d,%d) vertical neighbor\n", a+1, i+1, j+1);
                    }
                    if ( Sudoku[i][j].vPossibles[b] ) {
                        iNumChanges++;
                        Sudoku[i][j].vPossibles[b] = false;
                        printf("remove %d from cell(%d,%d) vertical neighbor\n", b+1, i+1, j+1);
                    }
                }
            }
        }
    }
    
    return ( iNumChanges > 0 );
}

bool
findNakedPairsGridNeighbors()
{
    int iNumChanges;
    int iNumPossibles;
    int iNumPairs;
    int Pairs[9][4];
    int iNumHiddenPairs;
    int HiddenPairs[9][6];
    int gridi, gridj, starti, startj, i, j, k, a, b, ii, jj, hp, i1, i2, j1, j2;
    bool foundMatch;

    iNumChanges = 0;
    for (gridi=0; gridi<3; gridi++) {
        for (gridj=0; gridj<3; gridj++) {
            if ( gridi == 2 && gridj == 2 ) {
                iNumPairs = 0;
            }
            starti = gridi * 3;
            startj = gridj * 3;
            iNumPairs = 0;
            iNumHiddenPairs = 0;
            for (i=starti; i<starti+3; i++) {
                for (j=startj; j<startj+3; j++) {
                    // look at only the unsolved cells ...
                    iNumPossibles = 0;
                    if ( Sudoku[i][j].iFound == NOTFOUND ) {
                        a = NOTFOUND;
                        b = NOTFOUND;
                        for (k=0; k<9; k++) {
                            if ( Sudoku[i][j].vPossibles[k] ) {
                                iNumPossibles++;
                                if ( a == NOTFOUND ) a = k;
                                else                 b = k;
                            }
                        }
                        if ( iNumPossibles == 2 ) {
                            Pairs[iNumPairs][0] = i;
                            Pairs[iNumPairs][1] = j;
                            Pairs[iNumPairs][2] = a;
                            Pairs[iNumPairs][3] = b;
                            iNumPairs++;
                        }
                    }
                }
            }
            if ( iNumPairs > 0 ) {
                for (k=0; k<iNumPairs; k++) {
                    printf("found pair( %d %d ) at cell(%d,%d)\n",
                           Pairs[k][2], Pairs[k][3],
                           Pairs[k][0], Pairs[k][1]);
                }
            }
            if ( iNumPairs > 1 ) {
                for (ii=0; ii<iNumPairs-1; ii++) {
                    for (jj=ii+1; jj<iNumPairs; jj++) {
                        foundMatch = ( Pairs[ii][2] == Pairs[jj][2] &&
                                       Pairs[ii][3] == Pairs[jj][3] );
                        if ( foundMatch ) {
                            printf("found a hidden pair( %d %d ) at cell(%d,%d) and cell(%d,%d) grid(%d,%d)\n",
                                   Pairs[ii][2]+1, Pairs[ii][3]+1,
                                   Pairs[ii][0]+1, Pairs[ii][1]+1,
                                   Pairs[jj][0]+1, Pairs[jj][1]+1,
                                   gridi+1, gridj+1);
                            a = Pairs[ii][2];
                            b = Pairs[ii][3];
                            HiddenPairs[iNumHiddenPairs][0] = Pairs[ii][0];
                            HiddenPairs[iNumHiddenPairs][1] = Pairs[ii][1];
                            HiddenPairs[iNumHiddenPairs][2] = Pairs[jj][0];
                            HiddenPairs[iNumHiddenPairs][3] = Pairs[jj][1];
                            HiddenPairs[iNumHiddenPairs][4] = Pairs[ii][2];
                            HiddenPairs[iNumHiddenPairs][5] = Pairs[ii][3];
                            iNumHiddenPairs++;
                        }
                    }
                }
            }
            if ( iNumHiddenPairs > 0 ) {
                // NEEDS WORK
                starti = gridi * 3;
                startj = gridj * 3;
                for (i=starti; i<starti+3; i++) {
                    for (j=startj; j<startj+3; j++) {
                        // for each hidedn pair
                        //    if (i1,j1) and (i2,j2) then skip
                        //    for pair (a,b)
                        //        if vPossible[a] then vPossible[a] = false
                        //        if vPossible[a] then vPossible[a] = false
                        for (hp=0; hp<iNumHiddenPairs; hp++) {
                            i1 = HiddenPairs[hp][0];
                            j1 = HiddenPairs[hp][1];
                            i2 = HiddenPairs[hp][2];
                            j2 = HiddenPairs[hp][3];
                            if ( (i == i1 && j == j1)|| (i == i2 && j == j2) ) continue;
                            a = HiddenPairs[hp][4];
                            b = HiddenPairs[hp][5];
                            if ( Sudoku[i][j].vPossibles[a] ) {
                                iNumChanges++;
                                Sudoku[i][j].vPossibles[a] = false;
                                printf("remove %d from cell(%d,%d) grid neighbor\n", a+1, i+1, j+1);
                            }
                            if ( Sudoku[i][j].vPossibles[b] ) {
                                iNumChanges++;
                                Sudoku[i][j].vPossibles[b] = false;
                                printf("remove %d from cell(%d,%d) grid neighbor\n", b+1, i+1, j+1);
                            }
                        }
                    }
                }
            }
        }
    }
    
    return ( iNumChanges > 0 );
}

bool
findNakedPairs()
{
    bool foundPair;
    foundPair = false;
    foundPair |= findNakedPairsHorizontalNeighbors();
    foundPair |= findNakedPairsVerticalNeighbors();
    foundPair |= findNakedPairsGridNeighbors();
    return foundPair;
}

bool
findLockedCandidateHorizontal()
{
    int i, j, k, startj, starti, ii, jj, gridj;
    int iNumChanges;
    bool Candidate[9][3][9];
    bool foundLockedCandidate;

    iNumChanges = 0;
    for (i=0; i<9; i++) {
        for (k=0; k<9; k++) {
            Candidate[i][0][k] = false;
            Candidate[i][1][k] = false;
            Candidate[i][2][k] = false;
        }
        for (k=0; k<9; k++) {
            for (j=0; j<9; j++) {
                if ( Sudoku[i][j].iFound == NOTFOUND ) {
                    gridj = j / 3;
                    Candidate[i][gridj][k] |= Sudoku[i][j].vPossibles[k];
                }
            }
        }
    }
    
    for (i=0; i<9; i++) {
        for (k=0; k<9; k++) {
            foundLockedCandidate = false;
            if ( Candidate[i][0][k] && !Candidate[i][1][k] && !Candidate[i][2][k] ) {
                foundLockedCandidate = true;
                startj = 0;
            }
            if ( !Candidate[i][0][k] && Candidate[i][1][k] && !Candidate[i][2][k] ) {
                foundLockedCandidate = true;
                startj = 3;
            }
            if ( !Candidate[i][0][k] && !Candidate[i][1][k] && Candidate[i][2][k] ) {
                foundLockedCandidate = true;
                startj = 6;
            }
            if ( foundLockedCandidate ) {
                printf("found locked candidated %d in row cells(%d,%d..%d)\n", k+1, i+1, startj+1, startj+3);
                starti = i / 3 * 3;
                for (ii=starti; ii<starti+3; ii++) {
                    if ( ii != i ) {
                        for (jj=startj; jj<startj+3; jj++) {
                            if ( Sudoku[ii][jj].vPossibles[k] ) {
                                iNumChanges++;
                                Sudoku[ii][jj].vPossibles[k] = false;
                                printf("Remove %d from cell(%d,%d) locked grid neighbor\n", k+1, ii+1, jj+1);
                            }
                        }
                    }
                }
            }
        }
    }
    
    return ( iNumChanges > 0 );
}

bool
findLockedCandidateVertical()
{
    //writeOutput();
    int i, j, k, startj, starti, ii, jj, gridi;
    int iNumChanges;
    bool Candidate[9][3][9];
    bool foundLockedCandidate;
    
    iNumChanges = 0;
    for (j=0; j<9; j++) {
        for (k=0; k<9; k++) {
            Candidate[j][0][k] = false;
            Candidate[j][1][k] = false;
            Candidate[j][2][k] = false;
        }
        for (k=0; k<9; k++) {
            for (i=0; i<9; i++) {
                if ( Sudoku[i][j].iFound == NOTFOUND ) {
                    gridi = i / 3;
                    Candidate[j][gridi][k] |= Sudoku[i][j].vPossibles[k];
                }
            }
        }
    }
    
    for (j=0; j<9; j++) {
        for (k=0; k<9; k++) {
            foundLockedCandidate = false;
            if ( Candidate[j][0][k] && !Candidate[j][1][k] && !Candidate[j][2][k] ) {
                foundLockedCandidate = true;
                starti = 0;
            }
            if ( !Candidate[j][0][k] && Candidate[j][1][k] && !Candidate[j][2][k] ) {
                foundLockedCandidate = true;
                starti = 3;
            }
            if ( !Candidate[j][0][k] && !Candidate[j][1][k] && Candidate[j][2][k] ) {
                foundLockedCandidate = true;
                starti = 6;
            }
            if ( foundLockedCandidate ) {
                printf("found locked candidated %d in column cells(%d..%d,%d)\n", k+1, starti+1, starti+3, j+1);
                startj = j / 3 * 3;
                for (jj=startj; jj<startj+3; jj++) {
                    if ( jj != j ) {
                        for (ii=starti; ii<starti+3; ii++) {
                            if ( Sudoku[ii][jj].vPossibles[k] ) {
                                iNumChanges++;
                                Sudoku[ii][jj].vPossibles[k] = false;
                                printf("Remove %d from cell(%d,%d) locked grid neighbor\n", k+1, ii+1, jj+1);
                            }
                        }
                    }
                }
            }
        }
    }
    
    return ( iNumChanges > 0 );
}

bool
findLockedCandidate()
{
    bool didSomething;
    didSomething = false;
    didSomething |= findLockedCandidateHorizontal();
    didSomething |= findLockedCandidateVertical();
    return didSomething;
}

int 
main(
    int argc, 
    const char * argv[]) 
{
    bool didSomething;

    convertInput(strInput);

    writeOutput();
    
    do {
        didSomething = findSingles();
        if ( ! didSomething ) {
            didSomething = findHiddenSingle();
            if ( ! didSomething ) {
                didSomething = findNakedPairs();
                if ( ! didSomething ) {
                    didSomething |= findLockedCandidate();
                }
            }
        }
    } while( didSomething );
    
    writeOutput();
    
    return 0;
}

