fn main() {
    let n = 3;
    let res = generate_parenthesis(n);
    println!("{:?}", res);
}

pub fn generate_parenthesis(n: i32) -> Vec<String> {
    /* I will keep the track of open parantheses and end it with 0 */
    let mut o_pt: u8 = 0; // cannot be negative
    let mut res = Vec::new();
    let mut ptrn = String::new();
    /* Programme starts with an initial parantheses pattern: (The most basic one)*/
    for i in 0..n {
        ptrn.push_str("()");
    } // -> It is ()()() for n = n
      // This will allow me to shuffle these ('s with )'s (and create every possible pattern)
      // Except the first '(' and the last ')'
    res.push(ptrn);
    res
}

fn shuffle(s: &str) -> Vec<String> {
    let b = s.as_bytes();
    let mut res: Vec<String> = vec![];
    for i in 1..b.len() - 1 {
        // we will only shuffle: _n_ => The middle.
        if b[i] == b'(' {}
    }
    res
}

fn find_next(pt: char, s: &str, idx: usize) -> usize {
    let b = s.as_bytes();
    for i in idx..b.len() {
            if b[i] == pt as u8 {
            return i;
        }
    }
    0
}
