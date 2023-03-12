/*

345. Reverse Vowels of a String
Easy
3.2K
2.4K
Companies
Given a string s, reverse only all the vowels in the string and return it.

The vowels are 'a', 'e', 'i', 'o', and 'u', and they can appear in both lower and upper cases, more than once.

 */


fn main() {
    pub fn reverse_vowels(s: String) -> String {
        let vowels = ['a', 'e', 'i', 'o', 'u'];

        let mut sb = &mut s.as_bytes();
        let len = sb.len();
        for i in 0..len {
            for j in 0..vowels.len() {
                if sb[i] == vowels[j] as u8 {
                    let tmp = sb[i];
                    let idx = len - i;
                    sb[i] = sb[idx];
                    sb[idx] = tmp;
                }
            }
        }

        let sb = match std::str::from_utf8(&mut sb) {
            Ok(i) => i,
            _     => panic!("invalid input"),
        };
        sb.to_owned()
    }
}
