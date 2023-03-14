/*


1935. Maximum Number of Words You Can Type
Easy
448
21
Companies
There is a malfunctioning keyboard where some letter keys do not work. All other keys on the keyboard work properly.

Given a string text of words separated by a single space (no leading or trailing spaces) and a string brokenLetters of all distinct letter keys that are broken, return the number of words in text you can fully type using this keyboard.


 */


fn main() {
    let text = "hello world".to_owned();
    let broken_letters = "ad".to_owned();

    let res = can_be_typed_words(text, broken_letters);

    // print!("{res}");
}


// 1ms
// impl Solution {
      pub fn can_be_typed_words(text: String, broken_letters: String) -> i32 {
        let mut broken = 0;
        text.split_ascii_whitespace().collect::<Vec<&str>>().iter().for_each(|&i| {
            if !i.chars().any(|j| broken_letters.chars().any(|k| k == j)) {
                broken += 1;
            }
        }
        );
        broken
    }
// }

// Fastest (0ms)
use std::collections::HashSet;
// impl Solution {
    pub fn can_be_typed_words(text: String, broken_letters: String) -> i32 {
       let mut broken_letters: HashSet<char> = {
           let mut set = HashSet::new();
           for broken_letter in broken_letters.chars() {
               set.insert(broken_letter);
           }
           set
       };

       let mut ans = 0;
       for word in text.split(" ") {
           if word.is_empty() {
               continue;
           }

           let mut passes = true;
           for c in word.chars() {
               if broken_letters.contains(&c) {
                   passes = false;
                   break;
               }
           }

           if passes {
                ans += 1;
           }
       }
       ans
    }
// }
