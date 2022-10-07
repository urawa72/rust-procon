use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        // O(n) solution
        let mut map = HashMap::with_capacity(nums.len());
        for (i, n) in nums.into_iter().enumerate() {
            let m = target - n;
            if let Some(&j) = map.get(&m) {
                return vec![j as i32, i as i32];
            } else {
                map.insert(n, i);
            }
        }

        // O(n^2) solution
        // for i in 0..nums.len() - 1 {
        //     for j in i + 1..nums.len() {
        //         if nums[i] + nums[j] == target {
        //             return vec![i as i32, j as i32];
        //         }
        //     }
        // }
        vec![]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
    }

    #[test]
    fn case2() {
        assert_eq!(Solution::two_sum(vec![3, 2, 4], 6), vec![1, 2]);
    }
}
