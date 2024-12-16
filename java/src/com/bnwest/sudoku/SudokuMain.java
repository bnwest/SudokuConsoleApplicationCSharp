/**
 * 
 */
package com.bnwest.sudoku;

import com.bnwest.sudoku.SudokuPuzzle;

/**
 * @author brian_000
 *
 */
public class SudokuMain {


    /**
	 * @param args
	 */
	public static void main(String[] args) {
		// TODO Auto-generated method stub
		 System.out.println("Solve Sudoku!");
		 System.out.println();
		 
		 SudokuPuzzle puzzle = new SudokuPuzzle();
		 puzzle.execute();
	}

}
