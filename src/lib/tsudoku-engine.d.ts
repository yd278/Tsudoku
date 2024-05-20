
declare module 'tsudoku-engine' {
    /**
     * Generates a string representation of a Sudoku grid based on a difficulty level.
     * @param difficulty - An integer representing the difficulty level of the grid.
     * @returns A string representation of the Sudoku grid.
     * @throws Will throw an error if the input is not a number.
     */
    export function generate(difficulty: number): string;
  
    /**
     * Finds the next step in a Sudoku solving process based on the current grid state.
     * @param gridPattern - A string representation of the current state of the Sudoku grid.
     * @returns A Node.js Buffer containing the next step data.
     * @throws Will throw an error if the input is not a string.
     */
    export function findNextStep(gridPattern: string): Buffer;
  }
  