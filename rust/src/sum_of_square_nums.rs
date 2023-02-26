/*
 * Given a non-negative integer c, decide whether there're two integers a and b such that a2 + b2 = c.
 *
 * Input: c = 5
 * Output: true
 * Explanation: 1 * 1 + 2 * 2 = 5
 */

fn main() {
    let c = 5;
    let expected = true;
    let found = judge_square_sum(c);
    assert_eq!(expected, found);
}

/* a^2 b^2 = c */
pub fn judge_square_sum(c: i32) -> bool {
    let mut l = 0i64;
    let mut r = (c as f64).sqrt() as i64;
    println!("{r}");
    while l <= r {
        let res: i64 = l * l + r * r;
        if res < c as i64 {
            l += 1;
        } else if res > c as i64 {
            r -= 1;
        } else {
            return true;
        }
    }
    false
}
