use proconio::{fastout, input};
use std::cmp::min;

#[fastout]
fn main() {
    input! { n: usize, h: [i64; n] }

    let mut dp = vec![1000000000i64; n + 10];
    dp[0] = 0;
    for i in 0..n - 1 {
        dp[i + 1] = min(dp[i] + (h[i + 1] - h[i]).abs(), dp[i + 1]);
        if i < n - 2 {
            dp[i + 2] = min(dp[i] + (h[i + 2] - h[i]).abs(), dp[i + 2]);
        }
    }

    println!("{}", dp[n - 1]);
}
