use proconio::{fastout, input};
use std::cmp::max;

#[fastout]
fn main() {
    input! { n: usize, w: usize, wv: [(usize, usize); n] }

    let mut dp = vec![vec![0usize; w + 100000]; n + 10];
    for i in 0..n {
        for j in 0..=w {
            dp[i + 1][j] = max(dp[i + 1][j], dp[i][j]);
            dp[i + 1][j + wv[i].0] = max(dp[i + 1][j + wv[i].0], dp[i][j] + wv[i].1);
        }
    }
    println!("{}", dp[n][w]);
}
