/*
1832. Check if the Sentence Is Pangram
Easy
2.2K
49
Companies
A pangram is a sentence where every letter of the English alphabet appears at least once.

Given a string sentence containing only lowercase English letters, return true if sentence is a pangram, or false otherwise.
*/

impl Solution {
    pub fn check_if_pangram(sentence: String) -> bool {
        let mut con = std::collections::HashSet::with_capacity(26);
        for bc in sentence.as_bytes() {
            if !con.contains(bc) {
                con.insert(bc);
            }
        }
        if con.len() == 26 {
            return true
        } else { return false }
    }
}

