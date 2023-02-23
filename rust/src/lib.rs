use Solution::*;

pub fn run() {
    //run_permute();
    run_reverse_string();
}

mod Solution {

    /*

    /*************************************************************************************************************************
     *Given an array nums of distinct integers, return all the possible permutations. You can return the answer in any order.*
     *************************************************************************************************************************/

    pub fn run_permute() {
        let nums = vec![1, 2, 3];
        let result = permute(nums);
        println!("{:?}", result);
    }
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = vec![];
        let mut nums = nums;
        let mut visited = vec![false; nums.len()];
        let mut path = vec![];
        backtrack(&mut result, &mut nums, &mut visited, &mut path);
        result
    }
    fn backtrack(result: &mut Vec<Vec<i32>>, nums: &mut Vec<i32>, visited: &mut Vec<bool>, path: &mut Vec<i32>) {
        if path.len() == nums.len() {
            result.push(path.clone());
            return;
        }
        for i in 0..nums.len() {
            if visited[i] {
                continue;
            }
            visited[i] = true;
            path.push(nums[i]);
            backtrack(result, nums, visited, path);
            path.pop();
            visited[i] = false;
        }
    }
    */

    use std::{mem::swap, ops::Index, str::Bytes};

    /*Given a string s, reverse the order of characters in each word within a sentence while still preserving whitespace and initial word order.
     *
     * Input: s = "Let's take LeetCode contest"
     * Output: "s'teL ekat edoCteeL tsetnoc"
     *
     * Easy
     * */

    pub fn run_reverse_string() {
        let inp = "Let's take LeetCode contest".to_owned();
        let out = "s'teL ekat edoCteeL tsetnoc".to_owned();
        let fn_out = reverse_words(inp);
        assert_eq!(out, fn_out);
    }
    pub fn reverse_words(s: String) -> String {
        let tmp;
        unsafe {
            tmp = s.as_bytes() as *const [u8] as *mut [u8];
        }

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
}
