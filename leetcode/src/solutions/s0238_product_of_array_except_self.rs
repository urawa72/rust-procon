pub struct Solution;

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let x = {
            let n = nums.iter().filter(|&n| n != &0).count();
            if n == 0 {
                0
            } else {
                nums.iter().filter(|&n| n != &0).product()
            }
        };
        let z = nums.iter().filter(|&n| n == &0).count();

        let judge = |n| {
            if (n != 0 && 0 < z) || (n == 0 && 1 < z) {
                0
            } else if n == 0 {
                x
            } else {
                x / n
            }
        };

        nums.into_iter().map(judge).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            Solution::product_except_self(vec![1, 2, 3, 4]),
            vec![24, 12, 8, 6]
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            Solution::product_except_self(vec![-1, 1, 0, -3, 3]),
            vec![0, 0, 9, 0, 0]
        );
    }

    #[test]
    fn case3() {
        assert_eq!(Solution::product_except_self(vec![0, 0]), vec![0, 0]);
    }

    #[test]
    fn case4() {
        assert_eq!(Solution::product_except_self(vec![0, 4, 0]), vec![0, 0, 0]);
    }
}
