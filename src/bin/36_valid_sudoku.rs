use std::collections::HashSet;

fn main() {
    let board = vec![
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
    let is_valid = Solution::is_valid_sudoku(board);
    println!("is valid sudoku ? {}", is_valid);
}

struct Solution;
impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut transposed = Self::transpose(&board);
        let mut grid = Self::grid(&board);
        let mut board = board;
        board.append(&mut transposed);
        board.append(&mut grid);
        for row in board {
            let mut row_seen = HashSet::new();
            for num in row {
                if num == '.' {
                    continue;
                }
                if !row_seen.insert(num) {
                    return false;
                }
            }
        }
        true
    }

    fn transpose(board: &Vec<Vec<char>>) -> Vec<Vec<char>> {
        let mut transposed = vec![vec!['.'; 9]; 9];
        for i in 0..9 {
            for j in 0..9 {
                transposed[j][i] = board[i][j];
            }
        }
        transposed
    }

    fn grid(board: &Vec<Vec<char>>) -> Vec<Vec<char>> {
        let mut grid = vec![vec!['.'; 9]; 9];
        for m in 0..9 {
            for n in 0..9 {
                let idx = (m / 3) * 3 + (n / 3);
                grid[idx].push(board[m][n])
            }
        }
        grid
    }
}
