// NOT WORKING

/*

50_f64. Pow(x, n)
Medium
6.8K
7.1K
Companies
Implement pow(x, n), which calculates x raised to the power n (i.e., xn).

 */


// impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        let mut res = 1_f64;

        if n == 0_i32 {
            return 1_f64;
        }
        if x == 0_f64 {
            return 0_f64;
        }
        if n > 0_i32 {
            if x > 0_f64 {
                for _ in 0..n {
                    res *= x;
                }
                return res;
            }
            if x < 0_f64 {
                if n % 2 == 0{
                    for _ in 0..n {
                        res *= x;
                    }
                    return res;
                } else {
                    for _ in 0..n {
                        res *= x;
                    }
                    return -res;
                }
            }
            return 0_f64;
        }
        if n < 0_i32{
            if n % 2 == 0_i32 {
                if x > 0_f64 {
                    for _ in 0..n {
                        res *= x;
                    }
                    return 1_f64/res;
                }
                if x < 0_f64 {
                    for _ in 0..n {
                        res *= x;
                    }
                    return 1_f64/res;
                }
            } else {
                if x > 0_f64 {
                    for _ in 0..n {
                        res *= x;
                    }
                    return 1_f64/res;
                }
                if x < 0_f64 {
                    for _ in 0..n {
                        res *= x;
                    }
                    return -1_f64/res;
                }
            }
            return 0_f64;
        }
        0_f64
    }
// }

// O(N)

// or x.powi(n) // lol
