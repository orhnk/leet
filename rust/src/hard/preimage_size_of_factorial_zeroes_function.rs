/* FIXME */
/*
793. Preimage Size of Factorial Zeroes Function
Hard
382
86
Companies
Let f(x) be the number of zeroes at the end of x!. Recall that x! = 1 * 2 * 3 * ... * x and by convention, 0! = 1.

For example, f(3) = 0 because 3! = 6 has no zeroes at the end, while f(11) = 2 because 11! = 39916800 has two zeroes at the end.
Given an integer k, return the number of non-negative integers x have the property that f(x) = k.
*/
impl Solution {
    /*
    // My take:
    pub fn preimage_size_fzf(k: i32) -> i32 {
        if k % 5 == 0 && k != 0 {
            return 0;
        }
        5
    }
    */

    // Just a runnable Solution (the only rust solution)

    pub fn preimage_size_fzf(k: i32) -> i32 {
        fn trailing_zeros(n: i64) -> i32 {
            let mut n = n;
            let mut answer = 0;
            while n > 0 {
                n /= 5;
                answer += n as i32;
            }
            answer
        }

        let mut lo = 0;
        let mut hi = 5 * (k as i64);
        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            let z = trailing_zeros(mid);
            if z < k {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }
        if trailing_zeros(lo) == k {
            5
        } else {
            0
        }
    }
}

/*
    First things first I want to visualize the problem:
    if we have number i for i! which is i / 5 == 1:
        we have a zero
        | i / 5 == j , we have "{j}" zeros

    we have the j value and we want to find the number of possible i's
    k = 0 | 0, 1, 2, 3, 4 ;; If we'd have a 5 there it'd mean a zero
    k = 1 | 5, 6, 7, 8, 9 ;; two zeros
    k = 2 | 10, 11, 12, 13, 14 ;; three zeros
    k = 3 | 15, 16, 17, 18, 19 ;; four  zeros
    k = 4 | 20, 21, 22, 23, 24 ;; five  zeros

    * This is the where it becomes handy.
    k = 5 | 25, 26, 27, 28, 29 ;; six   zeros
        ^   ^^
        |
    if we encounter a 5^n k its not valid to have a such possible i.
    But Why? because we are going to have one more 5 to append a zero
    to the end.

    Explanation:
        till k = 5 | we have got an accummilating linear 5's which will then get multiplicated with 2's to create zeros at the end.
        n = [N -> N % 5 != 0]
        k = 1 | 5 * n
        k = 2 | 5 * 5 * n
        k = 3 | 5 * 5 * 5 * n
        k = 4 | 5 * 5 * 5 * 5 * n
        k = 5 | 5 * 5 * 5 * 5 * 5 * 5 * n ;; Linearity broke because 25 has two 5's init
*/
