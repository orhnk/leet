#!/usr/bin/env run-cargo-script

fn main() {
    let s = "([)]".to_owned();
    let r = Solution::is_valid(s);

    println!("{r}");
}

struct Solution;
impl Solution {
    // First take:
    /*
    pub fn is_valid(s: String) -> bool {
        let s = unsafe { &mut *(&s as *const String as *mut String) };
        while s.contains("()") || s.contains("[]") || s.contains("{}") {
            *s = s.replace("()", "");
            *s = s.replace("[]", "");
            *s = s.replace("{}", "");
        }
        if s.len() == 0 {
            return true;
        }
        false
    }
    */

    // Second take: 
    pub fn is_valid(s: String) -> bool {
        let mut stack = Vec::new();
        for i in s.chars() {
            match i {
                '{' => stack.push('}'),
                '(' => stack.push(')'),
                '[' => stack.push(']'),
                '}'|')'|']' if Some(i) != stack.pop() => return false,
                _ => (),
            }
        }   
        return stack.is_empty();
    }

}
