/*
278. First Bad Version
Easy
7.1K
2.8K
Companies
You are a product manager and currently leading a team to develop a new product. Unfortunately, the latest version of your product fails the quality check. Since each version is developed based on the previous version, all the versions after a bad version are also bad.

Suppose you have n versions [1, 2, ..., n] and you want to find out the first bad one, which causes all the following ones to be bad.

You are given an API bool isBadVersion(version) which returns whether version is bad. Implement a function to find the first bad version. You should minimize the number of calls to the API.
*/

// The API isBadVersion is defined for you.
// isBadVersion(version:i32)-> bool;
// to call it use self.isBadVersion(version)


impl Solution {
    pub fn first_bad_version(&self, n: i32) -> i32 {
        let mut i = n / 2;
		    let is_bad = self.isBadVersion(b);
        if is_bad {
            i -= i/2;
        } else {
            i += i/2
        }
        self.first_bad_version(i)
    }
}
