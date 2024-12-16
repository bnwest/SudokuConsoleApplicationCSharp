/**
 * 
 */
package com.bnwest.sudoku;

/**
 * @author brian_000
 *
 */
public class SudokuCell {
    public static final int NOTFOUND = -1;

    public SudokuCell() {
        iFound = SudokuCell.NOTFOUND;
        vPossibilities = new boolean[9];
    }
    
    public int getFound() {
        return iFound;
    }
    public void setFound(int i) {
        iFound = i;
    }
    
    public boolean isPossible(int i) {
        return vPossibilities[i];
    }
    
    public void setPossible(int i) {
        setPossible(i, true);
    }

    public void setPossible(int i, boolean value) {
        vPossibilities[i] = value;
    }

    private int iFound;

	private boolean[] vPossibilities;
}
