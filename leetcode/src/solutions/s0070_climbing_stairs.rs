pub struct Solution {}

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let mut dp = vec![0i32; (n + 10) as usize];
        dp[0] = 1;
        for i in 0..n {
            let i = i as usize;
            dp[i + 1] += dp[i];
            dp[i + 2] += dp[i];
        }
        dp[n as usize]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn s0070_case1() {
        assert_eq!(2, Solution::climb_stairs(2));
    }

    #[test]
    fn s0070_case2() {
        assert_eq!(3, Solution::climb_stairs(3));
    }

    #[test]
    fn s0070_case3() {
        assert_eq!(17711, Solution::climb_stairs(21));
    }

    #[test]
    fn s0070_case4() {
        assert_eq!(1836311903, Solution::climb_stairs(45));
    }
}
