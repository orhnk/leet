
/*
1260. Shift 2D Grid
Easy
1.6K
322
Companies
Given a 2D grid of size m x n and an integer k. You need to shift the grid k times.

In one shift operation:

Element at grid[i][j] moves to grid[i][j + 1].
Element at grid[i][n - 1] moves to grid[i + 1][0].
Element at grid[m - 1][n - 1] moves to grid[0][0].
Return the 2D grid after applying shift operation k times.
*/


impl Solution {
    pub fn shift_grid(grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let len = grid.len();
        for i in 0..len {
            for j in 0..i {
                // Will shift grid k times.
                // swap(i, i+k) if i > offset : offset - i - k

                let (x, y) = (0, 0); // the grid[x][y]
                let (res, res_mod) = divide_mod(len, k);
                let (to_x, to_y) = (x + k/len, y +);
            }
        }
    }
    fn divide_mod(i: i32, j:i32) -> (i32, i32) {
        let res_mod = 0;
        let res = 0;
        while res_mod < j {
            i -= j;
            res += 1;
        }
        (res, res_mod)
    }
}
