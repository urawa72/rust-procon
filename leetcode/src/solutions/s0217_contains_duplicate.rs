use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut map = HashMap::new();
        for n in nums {
            map.entry(n).and_modify(|i| *i += 1).or_insert(1);
        }
        map.into_iter()
            .map(|e| e.1)
            .collect::<Vec<i32>>()
            .into_iter()
            .filter(|&i| i > 1)
            .count()
            > 0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        assert!(Solution::contains_duplicate(vec![1, 2, 3, 1]));
    }

    #[test]
    fn case2() {
        assert!(!Solution::contains_duplicate(vec![1, 2, 3]));
    }
}
