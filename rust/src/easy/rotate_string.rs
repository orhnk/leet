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

fn main() {
    let s = "pbxjbetnuarpyizxvgfgbrnhmiruxuhnyvvhawdnwoekxovasrmqacslhsgwzwdrljltnrqxvzhtlywyngwpkkxclnhkmreneduu".to_owned();
    let g = "nyvvhawdnwoekxovasrmqacslhsgwzwdrljltnrqxvzhtlywyngwpkkxclnhkmreneduupbxjbetnuarpyizxvgfgbrnhmiruxuh".to_owned();
    let e = true;
    let f = Solution::rotate_string(s, g);

    // assert_eq!(f, e);

    print!("{f}");
}

// This could be a bit of cheating. But The question did not indicate that was immutable (in python you can do it in the safe way :even in the fastest answer:)
struct Solution;

// This Solution was the both fastest and most memory efficient
impl Solution {
    pub fn rotate_string(s: String, goal: String) -> bool {
        unsafe {
            return s.len() == goal.len() && {
                // (*(&goal as *const String as *mut String)).push_str(&goal); // Unsafe mutability (kinda like interior mutability but more honest)
                let goal = format!("{}{}", goal, goal);
                goal.contains(&s)
            };
        }
    }
}
