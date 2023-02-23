
fn main() {
    let nums = vec![1, 2, 3];
    let result = permute(nums);
    println!("{:?}", result);
    assert_eq!(result, vec![vec![1, 2, 3], vec![1, 3, 2], vec![2, 1, 3], vec![2, 3, 1], vec![3, 1, 2], vec![3, 2, 1]]);
}

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
