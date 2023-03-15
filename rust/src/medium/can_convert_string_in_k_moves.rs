/*

1540. Can Convert String in K Moves
Medium
329
271
Companies
Given two strings s and t, your goal is to convert s into t in k moves or less.

During the ith (1 <= i <= k) move you can:

Choose any index j (1-indexed) from s, such that 1 <= j <= s.length and j has not been chosen in any previous move, and shift the character at that index i times.
Do nothing.
Shifting a character means replacing it by the next letter in the alphabet (wrapping around so that 'z' becomes 'a'). Shifting a character by i means applying the shift operations i times.

Remember that any index j can be picked at most once.

Return true if it's possible to convert s into t in no more than k moves, otherwise return false.

 */

// impl Solution {
    pub fn can_convert_string(s: String, t: String, k: i32) -> bool {
        /*This algorithm decreases k by the possiblity if k gets smaller than zero that means current state is impossible*/
        let mut moved = 0;
        let s = s.as_bytes();
        let t = t.as_bytes();
        let len = s.len();
        for i in 0..len {
            if s[i] != t[i] {
                moved += i32::abs(s[i] - t[i]);
                continue
            }
            if moved as i32 > k {
                return false;
            }
        }
        true
    }
// }
