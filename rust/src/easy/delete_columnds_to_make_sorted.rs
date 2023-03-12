

/*
944. Delete Columns to Make Sorted
Easy
1.5K
2.8K
Companies

You are given an array of n strings strs, all of the same length.

The strings can be arranged such that there is one on each line, making a grid.

    For example, strs = ["abc", "bce", "cae"] can be arranged as follows:

abc
bce
cae

You want to delete the columns that are not sorted lexicographically. In the above example (0-indexed), columns 0 ('a', 'b', 'c') and 2 ('c', 'e', 'e') are sorted, while column 1 ('b', 'c', 'a') is not, so you would delete column 1.

Return the number of columns that you will delete.
*/

fn main() {
    // for testing purpuses
    let res = b'a' > b'b';
    println!("{res}");
}
impl Solution {
    pub fn min_deletion_size(strs: Vec<String>) -> i32 {
        let mut res = 0;
        for i in 1..strs[0].len() {
            for j in 1..strs.len() {
                let bf = strs[j - 1].as_bytes();
                let bs = strs[j].as_bytes();
                if bf[i] > bs[i] {
                    res += 1;
                    break
                }
            }
        }
        res
    }
}

