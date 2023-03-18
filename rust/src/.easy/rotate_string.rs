/*
796. Rotate String
Easy
2.6K
108
Companies
Given two strings s and goal, return true if and only if s can become goal after some number of shifts on s.

A shift on s consists of moving the leftmost character of s to the rightmost position.

For example, if s = "abcde", then it will be "bcdea" after one shift.

 */

// Driver Code
fn main() {
    let s = "bqqutquvbtgouklsayfvzewpnrbwfcdmwctusunasdbpbmhnvy".to_string();
    let goal = "wpnrbwfcdmwctusunasdbpbmhnvybqqutquvbtgouklsayfvze".to_string();
    println!("{}", rotate_string(s, goal));
}

// impl Solution {
    pub fn rotate_string(s: String, goal: String) -> bool {

        if s.len() != goal.len() {
            return false;
        }

        let mut all = unsafe {&mut *(&s as *const String as *mut String)};

        dbg!(&all, &goal);
        all.push_str(&s);
        dbg!(&all, &goal);
        all.contains(&goal)
    }
// }
