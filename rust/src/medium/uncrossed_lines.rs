#!/usr/bin/env run-cargo-script

/*
1035. Uncrossed Lines
Medium
2K
28
Companies
You are given two integer arrays nums1 and nums2. We write the integers of nums1 and nums2 (in the order they are given) on two separate horizontal lines.

We may draw connecting lines: a straight line connecting two numbers nums1[i] and nums2[j] such that:

nums1[i] == nums2[j], and
the line we draw does not intersect any other connecting (non-horizontal) line.
Note that a connecting line cannot intersect even at the endpoints (i.e., each number can only belong to one connecting line).

Return the maximum number of connecting lines we can draw in this way.
*/

fn main() {
    let nums1 = vec![1, 4, 2];
    let nums2 = vec![1, 2, 4];
    let expected = 2;
    /* Explaining:
    1  4  2 *-> Cannot get crossed with the other 2 because it'd need to collide with the existing line 4 - 4
    |  |--+
    |     |
    1  2  4
     */
    let res = Solution::max_uncrossed_lines(nums1, nums2);

    print!("expected = {expected}, found = {res}");
}

struct Solution;
impl Solution {
    pub fn max_uncrossed_lines(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let len1 = nums1.len();
        let len2 = nums2.len();
        for i in 0..len1 {
            for j in 0..len2 {
                
            }
        }
    }
}
