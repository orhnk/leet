fn main() {
    let inp = "Let's take LeetCode contest".to_owned();
    let out = "s'teL ekat edoCteeL tsetnoc".to_owned();
    let fn_out = reverse_words(inp);
    assert_eq!(out, fn_out);
}

use std::mem::swap;

/*Given a string s, reverse the order of characters in each word within a sentence while still preserving whitespace and initial word order.
 *
 * Input: s = "Let's take LeetCode contest"
 * Output: "s'teL ekat edoCteeL tsetnoc"
 *
 * Easy
 * */

pub fn reverse_words(s: String) -> String {
    let tmp;
        tmp = s.as_bytes() as *const [u8] as *mut [u8];

    unsafe {
        let mut st = 0;
        let mut en = 0;
        //while en < s.len() {
        en = find_next_space(&*tmp, st).unwrap();
        reverse_string(&mut *tmp, st, en);
        st = en + 1;
        println!("{}", (*tmp).iter().map(|&c| c as char).collect::<String>());
        //}
    }

    println!("{}", find_next_space(s.as_bytes(), 0).unwrap());

    return String::new();
    fn find_next_space(s: &[u8], idx: usize) -> Result<usize, ()> {
        for i in idx..s.len() {
            if s[i] as char == ' ' {
                return Ok(i);
            }
        }
        Err(())
    }

    fn reverse_string(s: &mut [u8], idx_start: usize, idx_end: usize) {
        for i in idx_start..idx_end {
            s.swap(i, idx_end - i)
        }
    }
}
