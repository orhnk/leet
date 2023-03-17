/*
1526. Minimum Number of Increments on Subarrays to Form a Target Array
Hard
1.2K
59
Companies
You are given an integer array target. You have an integer array initial of the same size as target with all elements initially zeros.

In one operation you can choose any subarray from initial and increment each value by one.

Return the minimum number of operations to form a target array from initial.

The test cases are generated so that the answer fits in a 32-bit integer.
 */

// for testing purpuses this is the driver code:
fn main() {
    let target = vec![1,2,3,2,1];
    let res = min_number_operations(target);

    print!("{res}");
}


// TIME LIMIT EXCEEDED
// But works fine
// impl Solution {
pub fn min_number_operations_time_limit_exceeded(target: Vec<i32>) -> i32 {
    /* We assume that we start with [0, 0, 0, 0, 0 ... ] */
    let mut t = target.clone(); // cloning is pretty slow have to change that yeah.
    let mut moves = 0;
    let len = target.len();
    let mut started = false;
    while t.iter().any(|&i| i != 0) {
        for i in 0..len {
            if t[i] != 0 {
                t[i] -= 1;
                started = true;
                continue;
            }
            if started {
                break;
            }
        }
        moves += 1;
        started = false;
    }
    moves
}
pub fn min_number_operations(target: Vec<i32>) -> i32 {
    /* We assume that we start with [0, 0, 0, 0, 0 ... ] */
    let t = unsafe { &mut *(&target as *const Vec<i32> as *mut Vec<i32>) };
    let mut moves = 0;
    let len = t.len();
    let mut started = false;
    while t.iter().any(|&i| i != 0) {
        for i in 0..len {
            if t[i] != 0 {
                t[i] -= 1;
                started = true;
                continue;
            }
            if started {
                break;
            }
        }
        moves += 1;
        started = false;
    }
    moves
}
// }
