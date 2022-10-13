struct Solution;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }

        let mut nums = nums;
        nums.sort();

        let mut ans = 0;
        let mut cnt = 1;
        for i in 1..nums.len() {
            if nums[i] == nums[i - 1] + 1 {
                cnt += 1;
            } else if i - 1 != 0 && nums[i] == nums[i - 1] {
                continue;
            } else {
                ans = ans.max(cnt);
                cnt = 1;
            }
        }

        ans = ans.max(cnt);
        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(Solution::longest_consecutive(vec![100, 4, 200, 1, 3, 2]), 4);
    }

    #[test]
    fn case2() {
        assert_eq!(
            Solution::longest_consecutive(vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1]),
            9
        );
    }

    #[test]
    fn case3() {
        assert_eq!(Solution::longest_consecutive(vec![0]), 1);
    }

    #[test]
    fn case4() {
        assert_eq!(Solution::longest_consecutive(vec![]), 0);
    }

    #[test]
    fn case5() {
        assert_eq!(Solution::longest_consecutive(vec![1, 2, 0, 1]), 3);
    }
}
