pub struct Solution {}

// problem: https://leetcode.com/problems/sqrtx/
impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        use std::cmp::Ordering;
        if x <= 1 {
            return x;
        }

        let mut l = 0;
        let mut r = x / 2 + 1;
        let mut ans = 0;

        // Binary Search
        while l <= r {
            let m = (l + r) / 2;
            match m.cmp(&(x / m)) {
                Ordering::Greater => r = m - 1,
                Ordering::Equal => return m,
                Ordering::Less => {
                    l = m + 1;
                    ans = m;
                }
            }
        }
        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn s0069_case1() {
         assert_eq!(2, Solution::my_sqrt(4));
    }

    #[test]
    fn s0069_case2() {
         assert_eq!(2, Solution::my_sqrt(8));
    }

    #[test]
    fn s0069_case3() {
         assert_eq!(1, Solution::my_sqrt(1));
    }

    #[test]
    fn s0069_case4() {
         assert_eq!(0, Solution::my_sqrt(0));
    }
    #[test]
    fn s0069_case5() {
         assert_eq!(1728, Solution::my_sqrt(2988784));
    }
}
