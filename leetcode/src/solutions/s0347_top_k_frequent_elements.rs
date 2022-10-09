use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut map = HashMap::new();

        nums.into_iter()
            .for_each(|n| *map.entry(n).or_insert(0) += 1);
        let mut v: Vec<(i32, i32)> = map.into_iter().collect();
        v.sort_by(|a, b| b.1.cmp(&a.1));
        v.into_iter().map(|e| e.0).take(k as usize).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn case1() {
        assert_eq!(
            Solution::top_k_frequent(vec![1, 1, 1, 2, 2, 3], 2),
            vec![1, 2]
        );
    }

    #[test]
    fn case2() {
        assert_eq!(Solution::top_k_frequent(vec![1], 1), vec![1]);
    }
}
