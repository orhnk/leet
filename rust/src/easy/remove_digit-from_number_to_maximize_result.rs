/*

2259. Remove Digit From Number to Maximize Result
Easy
562
30
Companies
You are given a string number representing a positive integer and a character digit.

Return the resulting string after removing exactly one occurrence of digit from number such that the value of the resulting string in decimal form is maximized. The test cases are generated such that digit occurs at least once in number.

*/



impl Solution {
    pub fn remove_digit(number: String, digit: char) -> String {
        let mut res = String::new();
        let mut found = false;
        for c in number.chars() {
            if c == digit && !found {
                found = true;
            } else {
                res.push(c);
            }
        }
        res
    }
}
