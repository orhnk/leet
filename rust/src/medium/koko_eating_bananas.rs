/*

875. Koko Eating Bananas
Medium
7.3K
338
Companies
Koko loves to eat bananas. There are n piles of bananas, the ith pile has piles[i] bananas. The guards have gone and will come back in h hours.

Koko can decide her bananas-per-hour eating speed of k. Each hour, she chooses some pile of bananas and eats k bananas from that pile. If the pile has less than k bananas, she eats all of them instead and will not eat any more bananas during this hour.

Koko likes to eat slowly but still wants to finish eating all the bananas before the guards return.

Return the minimum integer k such that she can eat all the bananas within h hours.

 */

fn main() {
    let piles = vec![3, 6, 7, 11];
    let h = 4;
    println!("{}", min_eating_speed(piles, h));
}

// impl Solution {

// After! (
// Other rust solution from Solutions:
pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
    let (mut l, mut r) = (1, *piles.iter().max().unwrap());
    let mut res = r;
    while l <= r{
        let m = l + (r - l) / 2;
        let hours = piles.iter().fold(0i32, |acc, &p|  acc + (p + m -1)/m);
        if hours <= h{
            res = res.min(m);
            r = m - 1;
        }else {
            l = m + 1;
        }
    }
    res
}

// )

// Errorenous (Mine)
/* This is a binary search question which is "easy" to solve */
//                                            ^^^^- Arguably
pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
    let (mut l, mut r) = (0, piles.iter().max().unwrap());

    while l < *r {
        let m = l + (r - l) / 2;
        if dbg!(piles.iter().fold(0i32, |acc, &p|  acc + (p + m -1)/m)) <= h {
            let r = &m;
            continue
        }
        l = m;
    }
    l
}

// }
