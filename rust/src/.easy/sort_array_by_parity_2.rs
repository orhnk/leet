/*


922. Sort Array By Parity II
Easy
2.3K
83
Companies
Given an array of integers nums, half of the integers in nums are odd, and the other half are even.

Sort the array so that whenever nums[i] is odd, i is odd, and whenever nums[i] is even, i is even.

Return any answer array that satisfies this condition.

Input: nums = [4,2,5,7]
Output: [4,5,2,7]
Explanation: [4,7,2,5], [2,5,4,7], [2,7,4,5] would also have been accepted.



Output: [4 , 5 , 2 , 7]
         ^   ^   ^   ^
         |   |   |   |
        ---------------
       | 0 | 1 | 2 | 3 |
        ---------------
 */

fn main() {
    let input = [4,2,5,7];
    print!("{:?}", sort_array_by_parity_ii(input.to_vec()));
}




// impl Solution {
    pub fn sort_array_by_parity_ii(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        let len = nums.len();
        for i in 0..len {
            let is_same = (i + nums[i] as usize) % 2;
            if is_same != 0 {
                for j in i+1..len {
                    if (j + nums[j] as usize) % 2 != 0 && nums[j] % 2 != nums[i] % 2 {
                        let tmp = nums[i];
                        nums[i] = nums[j];
                        nums[j] = tmp;
                        break;
                    }
                }
            }
        }
        nums
    }
// }

// Iterator solution (Fastest) (6ms)

// impl Solution {
    pub fn sort_array_by_parity_ii(nums: Vec<i32>) -> Vec<i32> {
        let evens = nums.iter().filter(|x| *x % 2 == 0);
        let odds = nums.iter().filter(|x| *x % 2 != 0);

        Iterator::zip(evens, odds).fold(Vec::new(), |mut acc, (x, y)| {
            acc.push(*x);
            acc.push(*y);
            acc
        })
    }
// }

// Second (7ms)

// impl Solution {
    pub fn sort_array_by_parity_ii(nums: Vec<i32>) -> Vec<i32> {

        let (even, odd) : (Vec<_> , Vec<_>)= nums
                            .into_iter()
                            .partition(|x| x %2 == 0);
        Iterator::zip(even.into_iter(), odd.into_iter()).flat_map(|item| vec![item.0, item.1]).collect()
    }
// }
