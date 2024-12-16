/**
 * 
 */
package com.bnwest.sudoku;

import com.bnwest.sudoku.SudokuCell;

/**
 * @author brian_000
 *
 */
public class SudokuPuzzle {
    private SudokuCell[][] Sudoku; // = new SudokuCell[9][9];

    private String getInputString() {
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

    private void initializePuzzle() {
        int increment, currentIndex;

        String inputString = getInputString();
        
        // some input strings are 81 characters and 
        // some have an added plus sign between the rows.
        increment = ( inputString.charAt(9) == '+' ? 10 : 9 );
        
        Sudoku = new SudokuCell[9][];

        // convert one row at a time
        currentIndex = 0;
        for (int i=0; i<9; i++) {
            Sudoku[i] = new SudokuCell[9];

            String rowString = inputString.substring(currentIndex, currentIndex+9);
            for (int j=0; j<9; j++) {
                Sudoku[i][j] =  new SudokuCell();

                char cell = rowString.charAt(j);
                if ( '1' <= cell && cell <= '9' ) {
                    // starting the puzzle with this as a known value
                    int cellValue = cell - '1';
                    // yet we do not let the SudokuCell know that the value is known
                    // we will discover that later when we solve the puzzle.
                    Sudoku[i][j].setFound(SudokuCell.NOTFOUND);
                    for (int k=0; k<9; k++) Sudoku[i][j].setPossible(k, false);
                    Sudoku[i][j].setPossible(cellValue, true);
                }
                else {
                    Sudoku[i][j].setFound(SudokuCell.NOTFOUND);
                    for (int k=0; k<9; k++) Sudoku[i][j].setPossible(k, true);
                }
            }
            currentIndex += increment;
        }
    }
    
    public void displayPuzzle() {
//      "+-------+ +-------+ +-------+ +-------+ +-------+ +-------+ +-------+ +-------+ +-------+\n"
//      "| 1 2 3 | | 1 2 3 | | 1 2 3 | | 1 2 3 | | 1 2 3 | | 1 2 3 | | 1 2 3 | | 1 2 3 | | 1 2 3 |\n"
//      "| 4 5 6 | | 4 5 6 | | 4 5 6 | | 4 5 6 | | 4 5 6 | | 4 5 6 | | 4 5 6 | | 4 5 6 | | 4 5 6 |\n"
//      "| 7 8 9 | | 7 8 9 | | 7 8 9 | | 7 8 9 | | 7 8 9 | | 7 8 9 | | 7 8 9 | | 7 8 9 | | 7 8 9 |\n"
//      "+-------+ +-------+ +-------+ +-------+ +-------+ +-------+ +-------+ +-------+ +-------+\n"
        
        int i, j, k;
        for (i=0; i<9; i++) {
            System.out.println("+-------+ +-------+ +-------+  +-------+ +-------+ +-------+  +-------+ +-------+ +-------+");
            for (k=0; k<9; k+=3) {
                for (j=0; j<9; j++) {
                    System.out.format(
                        "| %c %c %c | ",
                        new Object[]
                            { new Character(Sudoku[i][j].isPossible(k)   ? Character.forDigit(k+1, 10) : ' '),
                              new Character(Sudoku[i][j].isPossible(k+1) ? Character.forDigit(k+2, 10) : ' '),
                              new Character(Sudoku[i][j].isPossible(k+2) ? Character.forDigit(k+3, 10) : ' ') }
                    );
                    if ( (j+1) % 3 == 0 ) {
                        System.out.print(" ");
                    }
                }
                System.out.println();
            }
            System.out.println("+-------+ +-------+ +-------+  +-------+ +-------+ +-------+  +-------+ +-------+ +-------+");
            if ( (i+1) % 3 == 0 ) {
                System.out.println();
            }
        }
    }

    public void displayPuzzle(String header) {
        System.out.println(header);
        displayPuzzle();
    } 
    
    public boolean removeHorizontalNeighbors(int i, int j, int k) {
        int column;
        boolean foundRemoval;

        foundRemoval = false;
        for (column=0; column<9; column++) {
            if ( column != j && Sudoku[i][column].isPossible(k) ) {
                foundRemoval = true;
                Sudoku[i][column].setPossible(k, false);
            }
        }

        return foundRemoval;
    }

    public boolean removeVerticalNeighbors(int i, int j, int k)
    {
        int row;
        boolean foundRemoval;

        foundRemoval = false;
        for (row=0; row<9; row++) {
            if ( row != i && Sudoku[row][j].isPossible(k) ) {
                foundRemoval = true;
                Sudoku[row][j].setPossible(k, false);
            }
        }
        
        return foundRemoval;
    }

    public boolean removeGridNeighbors(int i, int j, int k)
    {
        int starti, startj, ii, jj;
        boolean foundRemoval;
        
        foundRemoval = false;
        starti = i / 3 * 3;
        startj = j / 3 * 3;
        for (ii=starti; ii<starti+3; ii++) {
            for (jj=startj; jj<startj+3; jj++) {
                if ( ii != i && jj != j && Sudoku[ii][jj].isPossible(k) ) {
                    foundRemoval = true;
                    Sudoku[ii][jj].setPossible(k, false);
                }
            }
        }
        
        return foundRemoval;
    }

    public boolean removeNeighbors(int i, int j, int k)
    {
        boolean foundRemoval = false;
        foundRemoval |= removeHorizontalNeighbors(i, j, k);
        foundRemoval |= removeVerticalNeighbors  (i, j, k);
        foundRemoval |= removeGridNeighbors      (i, j, k);
        return foundRemoval;
    }

    public boolean findSingles()
    {
        int i, j, k;
        int iNumPossibilities;
        boolean foundSingle;
        boolean foundSingles;
        boolean foundRemoval;
        boolean didSomething;
        int iLastFound;
        int iValue;
        int[][] Singles = new int[9][9];

        didSomething = false;

        foundSingles = false;
        for (i = 0; i<9; i++) {
            for (j = 0; j<9; j++) {
                if ( Sudoku[i][j].getFound() == SudokuCell.NOTFOUND ) {
                    iNumPossibilities = 0;
                    iLastFound = SudokuCell.NOTFOUND;
                    for (k = 0; k<9; k++) {
                        if ( Sudoku[i][j].isPossible(k) ) {
                            iLastFound = k;
                            iNumPossibilities++;
                            if ( iNumPossibilities > 1 ) break;
                        }
                    }
                    foundSingle = ( iNumPossibilities == 1 );
                    if ( foundSingle ) {
                        foundSingles = true;
                        Singles[i][j] = iLastFound;
                        System.out.printf(
                                "found single %d at grid(%d,%d)\n", 
                                new Object[] { 
                                        new Integer(iLastFound+1), 
                                        new Integer(i+1), 
                                        new Integer(j+1) });
                    }
                    else {
                        Singles[i][j] = SudokuCell.NOTFOUND;
                    }

                }
                else {
                    Singles[i][j] = SudokuCell.NOTFOUND;
                }
            }
        }

        foundRemoval = false;
        if ( foundSingles ) {
            for (i = 0; i<9; i++) {
                for (j = 0; j<9; j++) {
                    foundSingle = ( Singles[i][j] != SudokuCell.NOTFOUND );
                    if ( foundSingle ) {
                        iValue = Singles[i][j];
                        // mark cell as a single
                        Sudoku[i][j].setFound(iValue);
                        foundRemoval = removeNeighbors(i, j, iValue);
                    }
                }
            }
            didSomething = foundRemoval;
        }

        return didSomething;
    }

    public boolean findHiddenSingle()
    {
        int i, j, k, gridi, gridj, starti, startj;
        int iNumPossibilities;
        int iLastFound, iLastFoundI, iLastFoundJ;
        boolean isSingleAlready;
        boolean foundHiddenSingle;
        boolean foundRemoval;
        int[][] HiddenSingle = new int[9][9];
        boolean didSomething;

        didSomething = false;

        foundHiddenSingle = false;
        for (i = 0; i<9; i++) {
            for (j = 0; j<9; j++) {
                for (k = 0; k<9; k++) {
                    HiddenSingle[i][j] = SudokuCell.NOTFOUND;
                }
            }
        }

        //
        // find hidden single in horizontal neighbor
        //
        
        for (i = 0; i<9; i++) {
            // look for k in each of the row's cells
            for (k = 0; k<9; k++) {
                iNumPossibilities = 0;
                iLastFound = SudokuCell.NOTFOUND;
                for (j = 0; j<9; j++) {
                    if ( Sudoku[i][j].isPossible(k) ) {
                        iNumPossibilities++;
                        iLastFound = j;
                        if ( iNumPossibilities > 1 ) break;
                    }
                }
                if ( iNumPossibilities == 1 ) {
                    isSingleAlready = ( Sudoku[i][iLastFound].getFound() != SudokuCell.NOTFOUND );
                    if ( !isSingleAlready ) {
                        HiddenSingle[i][iLastFound] = k;
                        foundHiddenSingle = true;
                        System.out.printf(
                                "found hidden single %d horizontally at grid(%d,%d)\n", 
                                new Object[] { 
                                        new Integer(k+1), 
                                        new Integer(i+1), 
                                        new Integer(iLastFound+1) });
                    }
                }
            }
        }

        //
        // find hidden single in vertical neighbor
        //
        
        for (j = 0; j<9; j++) {
            // look for k in each ofthe column's cells
            for (k = 0; k<9; k++) {
                iNumPossibilities = 0;
                iLastFound = SudokuCell.NOTFOUND;
                for (i = 0; i<9; i++) {
                    if ( Sudoku[i][j].isPossible(k) ) {
                        iNumPossibilities++;
                        iLastFound = i;
                        if ( iNumPossibilities > 1 ) break;
                    }
                }
                if ( iNumPossibilities == 1 ) {
                    isSingleAlready = ( Sudoku[iLastFound][j].getFound() != SudokuCell.NOTFOUND );
                    if ( !isSingleAlready ) {
                        if ( HiddenSingle[iLastFound][j] == SudokuCell.NOTFOUND ) {
                            HiddenSingle[iLastFound][j] = k;
                            foundHiddenSingle = true;
                            System.out.printf(
                                    "found hidden single %d vertically at grid(%d,%d)\n", 
                                    new Object[] { 
                                            new Integer(k+1), 
                                            new Integer(iLastFound+1), 
                                            new Integer(j+1) });
                        }
                    }
                }
            }
        }

        //
        // find hidden single in grid neighbor
        //
        
        for (gridi = 0; gridi<3; gridi++) {
            for (gridj = 0; gridj<3; gridj++) {
                starti = gridi * 3;
                startj = gridj * 3;
                for (k = 0; k<9; k++) {
                    iLastFoundI = SudokuCell.NOTFOUND;
                    iLastFoundJ = SudokuCell.NOTFOUND;
                    iNumPossibilities = 0;
                    for (i = starti; i<starti + 3; i++) {
                        for (j = startj; j<startj + 3; j++) {
                            if ( Sudoku[i][j].isPossible(k) ) {
                                iNumPossibilities++;
                                iLastFoundI = i;
                                iLastFoundJ = j;
                                if ( iNumPossibilities > 1 ) break;
                            }
                        }
                        if ( iNumPossibilities > 1 ) break;
                    }
                    if ( iNumPossibilities == 1 ) {
                        isSingleAlready = (Sudoku[iLastFoundI][iLastFoundJ].getFound() != SudokuCell.NOTFOUND);
                        if ( !isSingleAlready ) {
                            if ( HiddenSingle[iLastFoundI][iLastFoundJ] == SudokuCell.NOTFOUND ) {
                                HiddenSingle[iLastFoundI][iLastFoundJ] = k;
                                foundHiddenSingle = true;
                                System.out.printf(
                                        "found hidden single %d within the grid at grid(%d,%d)\n", 
                                        new Object[] { 
                                                new Integer(k+1), 
                                                new Integer(iLastFoundI+1), 
                                                new Integer(iLastFoundJ+1) });
                            }
                        }
                    }
                }
            }
        }

        foundRemoval = false;
        if ( foundHiddenSingle ) {
            for (i = 0; i<9; i++) {
                for (j = 0; j<9; j++) {
                    if ( HiddenSingle[i][j] != SudokuCell.NOTFOUND ) {
                        for (k = 0; k<9; k++) {
                            if ( k != HiddenSingle[i][j] ) {
                                Sudoku[i][j].setPossible(k, false);
                            }
                        }
                        Sudoku[i][j].setFound(HiddenSingle[i][j]);
                        foundRemoval = removeNeighbors(i, j, HiddenSingle[i][j]);
                    }
                }
            }
            didSomething = foundRemoval;
        }

        return didSomething;
    }

    public boolean findNakedPairsHorizontalNeighbors()
    {
        int i, j, k, a, b, ii, jj, hp, j1, j2;
        int iNumPossibles;
        boolean foundMatch;
        int iNumChanges;

        int iNumPairs;
        int[][] Pairs = new int[9][4];       // better name CandiatePairs[][]
        int iNumHiddenPairs;
        int[][] HiddenPairs = new int[9][6]; // better name NakedPairs[][]

        iNumChanges = 0;
        for (i = 0; i<9; i++) {
            // for each row
            iNumPairs = 0;
            iNumHiddenPairs = 0;
            for (j = 0; j<9; j++) {
                // look at only the unsolved cells ...
                iNumPossibles = 0;
                if ( Sudoku[i][j].getFound() == SudokuCell.NOTFOUND ) {
                    a = SudokuCell.NOTFOUND;
                    b = SudokuCell.NOTFOUND;
                    for (k = 0; k<9; k++) {
                        if ( Sudoku[i][j].isPossible(k) ) {
                            iNumPossibles++;
                            if ( a == SudokuCell.NOTFOUND ) a = k;
                            else                            b = k;
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
                for (k = 0; k<iNumPairs; k++) {
                    System.out.printf(
                            "found candidate pair( %d %d ) at cell(%d,%d)\n", 
                            new Object[] {
                                    new Integer(Pairs[k][2]+1), new Integer(Pairs[k][3]+1), 
                                    new Integer(Pairs[k][0]+1), new Integer(Pairs[k][1]+1) });
                }
            }
            if ( iNumPairs > 1 ) {
                for (ii = 0; ii<iNumPairs - 1; ii++) {
                    for (jj = ii + 1; jj<iNumPairs; jj++) {
                        foundMatch = ( Pairs[ii][2] == Pairs[jj][2] &&
                                       Pairs[ii][3] == Pairs[jj][3] );
                        if ( foundMatch ) {
                            System.out.printf(
                                    "found a pair( %d %d ) at cell(%d,%d) and cell(%d,%d) row %d\n", 
                                    new Object[] { 
                                            new Integer(Pairs[ii][2]+1), new Integer(Pairs[ii][3]+1), 
                                            new Integer(Pairs[ii][0]+1), new Integer(Pairs[ii][1]+1), 
                                            new Integer(Pairs[jj][0]+1), new Integer(Pairs[jj][1]+1),
                                            new Integer(i+1) });
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
                for (j = 0; j<9; j++) {
                    // for each hidedn pair
                    //    if (i1,j1) and (i2,j2) then skip
                    //    for pair (a,b)
                    //        if vPossible[a] then vPossible[a] = false
                    //        if vPossible[a] then vPossible[a] = false
                    for (hp = 0; hp<iNumHiddenPairs; hp++) {
                        j1 = HiddenPairs[hp][1];
                        j2 = HiddenPairs[hp][3];
                        boolean lookingAtPairCell = ( j == j1 || j == j2 );
                        if ( lookingAtPairCell ) continue;
                        a = HiddenPairs[hp][4];
                        b = HiddenPairs[hp][5];
                        if ( Sudoku[i][j].isPossible(a) ) {
                            iNumChanges++;
                            Sudoku[i][j].setPossible(a, false);
//                            printf("remove %d from cell(%d,%d) horizontal neighbor\n", a + 1, i + 1, j + 1);
                        }
                        if ( Sudoku[i][j].isPossible(b) ) {
                            iNumChanges++;
                            Sudoku[i][j].setPossible(b, false);
//                            printf("remove %d from cell(%d,%d) horizontal neighbor\n", b + 1, i + 1, j + 1);
                        }
                    }
                }
            }
        }

        return (iNumChanges > 0);
    }

    public boolean findNakedPairsVerticalNeighbors()
    {
        int i, j, k, a, b, ii, jj, hp, i1, i2;
        int iNumPossibles;
        boolean foundMatch;
        int iNumChanges;

        int iNumPairs;
        int[][] Pairs = new int[9][4];       // better name CandiatePairs[][]
        int iNumHiddenPairs;
        int[][] HiddenPairs = new int[9][6]; // better name NakedPairs[][]

        iNumChanges = 0;
        for (j = 0; j<9; j++) {
            // for each column
            iNumPairs = 0;
            iNumHiddenPairs = 0;
            for (i = 0; i<9; i++) {
                // look at only the unsolved cells ...
                iNumPossibles = 0;
                if ( Sudoku[i][j].getFound() == SudokuCell.NOTFOUND ) {
                    a = SudokuCell.NOTFOUND;
                    b = SudokuCell.NOTFOUND;
                    for (k = 0; k<9; k++) {
                        if ( Sudoku[i][j].isPossible(k) ) {
                            iNumPossibles++;
                            if (a == SudokuCell.NOTFOUND) a = k;
                            else                          b = k;
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
            if (iNumPairs > 0) {
                for (k = 0; k<iNumPairs; k++) {
                    System.out.printf(
                            "found candidate pair( %d %d ) at cell(%d,%d)\n", 
                            new Object[] {
                                    new Integer(Pairs[k][2]+1), new Integer(Pairs[k][3]+1), 
                                    new Integer(Pairs[k][0]+1), new Integer(Pairs[k][1]+1) });
                }
            }
            if ( iNumPairs > 1 ) {
                for (ii = 0; ii<iNumPairs - 1; ii++) {
                    for (jj = ii + 1; jj<iNumPairs; jj++) {
                        foundMatch = (Pairs[ii][2] == Pairs[jj][2] &&
                            Pairs[ii][3] == Pairs[jj][3]);
                        if ( foundMatch ) {
                            System.out.printf(
                                    "found a pair( %d %d ) at cell(%d,%d) and cell(%d,%d) column %d\n", 
                                    new Object[] { 
                                            new Integer(Pairs[ii][2]+1), new Integer(Pairs[ii][3]+1), 
                                            new Integer(Pairs[ii][0]+1), new Integer(Pairs[ii][1]+1), 
                                            new Integer(Pairs[jj][0]+1), new Integer(Pairs[jj][1]+1),
                                            new Integer(j+1) });
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
                for (i = 0; i<9; i++) {
                    // for each hidedn pair
                    //    if (i1,j1) and (i2,j2) then skip
                    //    for pair (a,b)
                    //        if vPossible[a] then vPossible[a] = false
                    //        if vPossible[a] then vPossible[a] = false
                    for (hp = 0; hp<iNumHiddenPairs; hp++) {
                        i1 = HiddenPairs[hp][0];
                        i2 = HiddenPairs[hp][2];
                        boolean lookingAtPairCell = ( i == i1 || i == i2 );
                        if ( lookingAtPairCell ) continue;
                        a = HiddenPairs[hp][4];
                        b = HiddenPairs[hp][5];
                        if ( Sudoku[i][j].isPossible(a) ) {
                            iNumChanges++;
                            Sudoku[i][j].setPossible(a, false);
                            //printf("remove %d from cell(%d,%d) vertical neighbor\n", a + 1, i + 1, j + 1);
                        }
                        if ( Sudoku[i][j].isPossible(b) ) {
                            iNumChanges++;
                            Sudoku[i][j].setPossible(b, false);
                            //printf("remove %d from cell(%d,%d) vertical neighbor\n", b + 1, i + 1, j + 1);
                        }
                    }
                }
            }
        }

        return (iNumChanges > 0);
    }

    public boolean findNakedPairsGridNeighbors()
    {
        int iNumChanges;
        int iNumPossibles;

        int iNumPairs;
        int[][] Pairs = new int[9][4];       // better name CandiatePairs[][]
        int iNumHiddenPairs;
        int[][] HiddenPairs = new int[9][6]; // better name NakedPairs[][]

        int gridi, gridj, starti, startj, i, j, k, a, b, ii, jj, hp, i1, i2, j1, j2;
        boolean foundMatch;

        iNumChanges = 0;
        for (gridi = 0; gridi<3; gridi++) {
            for (gridj = 0; gridj<3; gridj++) {
                if ( gridi == 2 && gridj == 2 ) {
                    iNumPairs = 0;
                }
                starti = gridi * 3;
                startj = gridj * 3;
                iNumPairs = 0;
                iNumHiddenPairs = 0;
                for (i = starti; i<starti + 3; i++) {
                    for (j = startj; j<startj + 3; j++) {
                        // look at only the unsolved cells ...
                        iNumPossibles = 0;
                        if ( Sudoku[i][j].getFound() == SudokuCell.NOTFOUND ) {
                            a = SudokuCell.NOTFOUND;
                            b = SudokuCell.NOTFOUND;
                            for (k = 0; k<9; k++) {
                                if ( Sudoku[i][j].isPossible(k) ) {
                                    iNumPossibles++;
                                    if ( a == SudokuCell.NOTFOUND ) a = k;
                                    else                            b = k;
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
                    for (k = 0; k<iNumPairs; k++) {
//                        printf("found pair( %d %d ) at cell(%d,%d)\n",
//                            Pairs[k][2], Pairs[k][3],
//                            Pairs[k][0], Pairs[k][1]);
                        System.out.printf(
                                "found candidate pair( %d %d ) at cell(%d,%d)\n", 
                                new Object[] {
                                        new Integer(Pairs[k][2]+1), new Integer(Pairs[k][3]+1), 
                                        new Integer(Pairs[k][0]+1), new Integer(Pairs[k][1]+1) });
                    }
                }
                if ( iNumPairs > 1 ) {
                    for (ii = 0; ii<iNumPairs - 1; ii++) {
                        for (jj = ii + 1; jj<iNumPairs; jj++) {
                            foundMatch = ( Pairs[ii][2] == Pairs[jj][2] &&
                                           Pairs[ii][3] == Pairs[jj][3] );
                            if ( foundMatch ) {
                                System.out.printf(
                                        "found a pair( %d %d ) at cell(%d,%d) and cell(%d,%d) grid(%d,%d)\n", 
                                        new Object[] { 
                                                new Integer(Pairs[ii][2]+1), new Integer(Pairs[ii][3]+1), 
                                                new Integer(Pairs[ii][0]+1), new Integer(Pairs[ii][1]+1), 
                                                new Integer(Pairs[jj][0]+1), new Integer(Pairs[jj][1]+1),
                                                new Integer(gridi+1), new Integer(gridj+1) });
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
                    for (i = starti; i<starti + 3; i++) {
                        for (j = startj; j<startj + 3; j++) {
                            // for each hidedn pair
                            //    if (i1,j1) and (i2,j2) then skip
                            //    for pair (a,b)
                            //        if vPossible[a] then vPossible[a] = false
                            //        if vPossible[a] then vPossible[a] = false
                            for (hp = 0; hp<iNumHiddenPairs; hp++) {
                                i1 = HiddenPairs[hp][0];
                                j1 = HiddenPairs[hp][1];
                                i2 = HiddenPairs[hp][2];
                                j2 = HiddenPairs[hp][3];
                                boolean lookingAtPairCell = ((i == i1 && j == j1) || (i == i2 && j == j2));
                                if ( lookingAtPairCell ) continue;
                                a = HiddenPairs[hp][4];
                                b = HiddenPairs[hp][5];
                                if ( Sudoku[i][j].isPossible(a) ) {
                                    iNumChanges++;
                                    Sudoku[i][j].setPossible(a, false);
                                    //printf("remove %d from cell(%d,%d) grid neighbor\n", a + 1, i + 1, j + 1);
                                }
                                if ( Sudoku[i][j].isPossible(b) ) {
                                    iNumChanges++;
                                    Sudoku[i][j].setPossible(b, false);
                                    //printf("remove %d from cell(%d,%d) grid neighbor\n", b + 1, i + 1, j + 1);
                                }
                            }
                        }
                    }
                }
            }
        }

        return (iNumChanges > 0);
    }

    public boolean findNakedPairs()
    {
        boolean foundPair;
        foundPair = false;
        foundPair |= findNakedPairsHorizontalNeighbors();
        foundPair |= findNakedPairsVerticalNeighbors();
        foundPair |= findNakedPairsGridNeighbors();
        return foundPair;
    }

    public boolean findLockedCandidateHorizontal()
    {
        int i, j, k, startj, starti, ii, jj, gridj;
        int iNumChanges;
        boolean[][][] Candidate = new boolean[9][3][9];
        boolean foundLockedCandidate;

        iNumChanges = 0;
        for (i = 0; i<9; i++) {
            for (k = 0; k<9; k++) {
                Candidate[i][0][k] = false;
                Candidate[i][1][k] = false;
                Candidate[i][2][k] = false;
            }
            for (k = 0; k<9; k++) {
                for (j = 0; j<9; j++) {
                    if ( Sudoku[i][j].getFound() == SudokuCell.NOTFOUND ) {
                        gridj = j / 3;
                        Candidate[i][gridj][k] |= Sudoku[i][j].isPossible(k);
                    }
                }
            }
        }

        for (i = 0; i<9; i++) {
            for (k = 0; k<9; k++) {
                foundLockedCandidate = false;
                startj = SudokuCell.NOTFOUND; // Java compiler guesses below that startj may not be initialized
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
                    System.out.printf(
                            "found locked candidated %d in row cells(%d,%d..%d)\n", 
                            new Object[] { 
                                    new Integer(k+1), 
                                    new Integer(i+1), 
                                    new Integer(startj+1), new Integer(startj+3) });
                    starti = i / 3 * 3;
                    for ( ii = starti; ii<starti + 3; ii++ ) {
                        if (ii != i) {
                            for (jj = startj; jj<startj + 3; jj++) {
                                if ( Sudoku[ii][jj].isPossible(k) ) {
                                    iNumChanges++;
                                    Sudoku[ii][jj].setPossible(k, false);
                                    //printf("Remove %d from cell(%d,%d) locked grid neighbor\n", k + 1, ii + 1, jj + 1);
                                }
                            }
                        }
                    }
                }
            }
        }

        return ( iNumChanges > 0 );
    }

    public boolean findLockedCandidateVertical()
    {
        //writeOutput();
        int i, j, k, startj, starti, ii, jj, gridi;
        int iNumChanges;
        boolean[][][] Candidate = new boolean[9][3][9];
        boolean foundLockedCandidate;

        iNumChanges = 0;
        for (j = 0; j<9; j++) {
            for (k = 0; k<9; k++) {
                Candidate[j][0][k] = false;
                Candidate[j][1][k] = false;
                Candidate[j][2][k] = false;
            }
            for (k = 0; k<9; k++) {
                for (i = 0; i<9; i++) {
                    if ( Sudoku[i][j].getFound() == SudokuCell.NOTFOUND ) {
                        gridi = i / 3;
                        Candidate[j][gridi][k] |= Sudoku[i][j].isPossible(k);
                    }
                }
            }
        }

        for (j = 0; j<9; j++) {
            for (k = 0; k<9; k++) {
                foundLockedCandidate = false;
                starti = SudokuCell.NOTFOUND; // Java compiler guesses below that starti may not be initialized
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
//                    printf("found locked candidated %d in column cells(%d..%d,%d)\n", 
//                            k + 1, starti + 1, starti + 3, j + 1);
                    System.out.printf(
                            "found locked candidated %d in column cells(%d..%d,%d)\n", 
                            new Object[] { 
                                    new Integer(k+1), 
                                    new Integer(starti+1), new Integer(starti+3), 
                                    new Integer(j+1) });
                    displayPuzzle();
                    startj = j / 3 * 3;
                    for (jj = startj; jj<startj + 3; jj++) {
                        if (jj != j) {
                            for (ii = starti; ii<starti + 3; ii++) {
                                if ( Sudoku[ii][jj].isPossible(k) ) {
                                    iNumChanges++;
                                    Sudoku[ii][jj].setPossible(k, false);
                                    //printf("Remove %d from cell(%d,%d) locked grid neighbor\n", k + 1, ii + 1, jj + 1);
                                }
                            }
                        }
                    }
                }
            }
        }

        return ( iNumChanges > 0 );
    }

    public boolean findLockedCandidate()
    {
        boolean didSomething;
        didSomething = false;
        didSomething |= findLockedCandidateHorizontal();
        didSomething |= findLockedCandidateVertical();
        return didSomething;
    }

    public void solvePuzzle() {
        boolean didSomething;

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
    }

    public void execute() {
        initializePuzzle();
        displayPuzzle("Start of Puzzle:");
        solvePuzzle();
        displayPuzzle("End of Puzzle:");
    }
}
