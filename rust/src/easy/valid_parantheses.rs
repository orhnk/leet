#!/usr/bin/env run-cargo-script
impl Solution {
    pub fn is_valid(s: String) -> bool {
        let (mut b, mut c, mut sq) = (0, 0, 0); // braces, square " ", curly " "
        s.chars().for_each(|i| {
            if i == '(' {
                b += 1;
            }
            if i == ')' {
                b -= 1;
            }
            if i == '{' {
                c += 1;
            }
            if i == '}' {
                c -= 1;
            }
            if i == '[' {
                sq += 1;
            }
            if i == ']' {
                sq -= 1;
            }
        });
    if b == 0 && c == 0 && sq == 0 {
        return true;
    }
    false
    }
}
