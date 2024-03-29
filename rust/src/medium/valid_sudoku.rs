/*
36. Valid Sudoku
Medium
8.2K
884
Companies
Determine if a 9 x 9 Sudoku board is valid. Only the filled cells need to be validated according to the following rules:

Each row must contain the digits 1-9 without repetition.
Each column must contain the digits 1-9 without repetition.
Each of the nine 3 x 3 sub-boxes of the grid must contain the digits 1-9 without repetition.
Note:

A Sudoku board (partially filled) could be valid but is not necessarily solvable.
Only the filled cells need to be validated according to the mentioned rules.
 */

fn main() {
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
    let res = Solution::is_valid_sudoku(board);
    print!("{res}");
}

struct Solution();

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        /* This program will look at the rows coulumns and 3x3 cubes and will search for dupes. If there is a dupe It will return false, else true */
        let len = board.len(); // Becuase board is a square It's fine to have only one len variable.
        for i in 0..len {
            // if the coulumns does not include any dupe
            if Self::has_dupe_ver(&board, i, len) {
                return false;
            }

            // if the rows does not include any dupe
            if Self::has_dupe_hor(&board, i, len) {
                return false;
            }
        }

        // If cubes does not include dupe
        let slots = len / 3;
        for i in 0..slots {
            for j in 0..slots {
                if Self::has_dupe_cube(&board, i * 3, j * 3, slots) {
                    return false;
                }
            }
        }

        true
    }

    fn has_dupe_cube(s: &Vec<Vec<char>>, k: usize, l: usize, slots: usize) -> bool {
        let mut set = std::collections::HashSet::new();
        for i in 0..slots {
            for j in 0..slots {
                if !set.contains(&s[i + k][j + l]) {
                    set.insert(&s[i + k][j + l]);
                } else if s[i + k][j + l] != '.' {
                    return true;
                }
            }
        }
        false
    }

    fn has_dupe_ver(s: &Vec<Vec<char>>, n: usize, len: usize) -> bool {
        let mut set = std::collections::HashSet::with_capacity(len);
        for i in 0..len {
            if !set.contains(&s[i][n]) {
                set.insert(s[i][n]);
            } else if s[i][n] != '.' {
                return true;
            }
        }
        false
    }

    fn has_dupe_hor(s: &Vec<Vec<char>>, n: usize, len: usize) -> bool {
        let mut set = std::collections::HashSet::with_capacity(len);
        for i in 0..len {
            if !set.contains(&s[n][i]) {
                set.insert(s[n][i]);
            } else if s[n][i] != '.' {
                return true;
            }
        }
        false
    }
}
