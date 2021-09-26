use proconio::{fastout, input};
use std::cmp::*;

#[fastout]
fn main() {
    input! { n: usize, x: usize, y: usize, ab: [(usize, usize); n]  }
    let mut dp = vec![vec![vec![1000; y + 1]; x + 1]; n + 1];

    dp[0][0][0] = 0;
    for i in 0..n {
        for j in 0..=x {
            for k in 0..=y {
                let ja = min(j + ab[i].0, x);
                let kb = min(k + ab[i].1, y);
                dp[i + 1][ja][kb] = min(dp[i + 1][ja][kb], dp[i][j][k] + 1);
                dp[i + 1][j][k] = min(dp[i + 1][j][k], dp[i][j][k]);
            }
        }
    }

    if dp[n][x][y] >= 1000 {
        println!("{}", -1);
    } else {
        println!("{}", dp[n][x][y]);
    }
}
