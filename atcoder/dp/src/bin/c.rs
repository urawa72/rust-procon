use proconio::{input, fastout};
use std::cmp::max;

#[fastout]
fn main() {
    input! { n: usize, v: [[usize; 3]; n] }
    let mut dp = vec![vec![0; 3]; n + 10];
    for i in 0..n {
        dp[i + 1][0] = max(dp[i][1] + v[i][1], dp[i][2] + v[i][2]);
        dp[i + 1][1] = max(dp[i][0] + v[i][0], dp[i][2] + v[i][2]);
        dp[i + 1][2] = max(dp[i][1] + v[i][1], dp[i][0] + v[i][0]);
    }

    println!("{}", max(dp[n][2], max(dp[n][0], dp[n][1])));
}
