pub struct Solution;

impl Solution {
    /// Determines if a 9x9 Sudoku board is valid.
    /// Only the filled cells need to be validated according to the following rules:
    /// 1. Each row must contain the digits 1-9 without repetition.
    /// 2. Each column must contain the digits 1-9 without repetition.
    /// 3. Each of the nine 3x3 sub-boxes must contain the digits 1-9 without repetition.
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        // Use bitmasks to track digits seen in each row, column, and 3x3 box.
        // Each bit in the u16 represents a digit from 1 to 9.
        let mut rows = [0u16; 9];
        let mut cols = [0u16; 9];
        let mut boxes = [0u16; 9];

        for r in 0..9 {
            for c in 0..9 {
                let ch = board[r][c];
                if ch == '.' {
                    continue;
                }

                // Convert char digit to 0-indexed number (0-8)
                let num = (ch as u8) - b'1';
                let mask = 1 << num;
                
                // Calculate which 3x3 sub-box the current cell belongs to
                let box_idx = (r / 3) * 3 + (c / 3);

                // Check if the digit has already appeared in the current row, column, or box
                if (rows[r] & mask) != 0 || (cols[c] & mask) != 0 || (boxes[box_idx] & mask) != 0 {
                    return false;
                }

                // Mark the digit as seen by updating the masks
                rows[r] |= mask;
                cols[c] |= mask;
                boxes[box_idx] |= mask;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_sudoku() {
        let board = vec![
            vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];
        assert!(Solution::is_valid_sudoku(board));

        let board2 = vec![
            vec!['8', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];
        // '8' is repeated in the first column
        assert!(!Solution::is_valid_sudoku(board2));
    }
}
