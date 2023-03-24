#!/bin/sh
//usr/bin/env rustc $0 -o a.out && ./a.out && rm ./a.out ; exit

/*
167. Two Sum II - Input Array Is Sorted
Medium
9.1K
1.2K
Companies
Given a 1-indexed array of integers numbers that is already sorted in non-decreasing order, find two numbers such that they add up to a specific target number. Let these two numbers be numbers[index1] and numbers[index2] where 1 <= index1 < index2 <= numbers.length.

Return the indices of the two numbers, index1 and index2, added by one as an integer array [index1, index2] of length 2.

The tests are generated such that there is exactly one solution. You may not use the same element twice.

Your solution must use only constant extra space.
*/

fn main() {
    let target = 8;
    let numbers = [1,2,3]
    println!("{:?}", Solution::two_sum(numbers, target));
}

struct Solution;

impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        /* This Solution uses binary search */

        let len = numbers.len();

        for i in 0..len {
            let (mut l, mut r) = (0, len - 1);
            while l <= r {
                let mut m = l + (r - l) / 2;
                let res = numbers[i] + numbers[m];
                if res > target {
                    r = m - 1;
                    continue;
                }
                if res < target || i == m {
                    l = m + 1;
                    continue;
                }
                return vec![i as i32 + 1, m as i32 + 1];
            }
        }
        vec![-1, -1]
    }
}
