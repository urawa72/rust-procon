use proconio::{fastout, input};
use std::cmp::*;

const MOD: isize = 1_000_000_007;

#[fastout]
fn main() {
    input! { n: usize, k: usize, a: [usize; n] }

    // dp[i][j] := i人目までにj個配る組み合わせ
    let mut dp = vec![vec![0isize; k + 10]; n + 10];

    dp[0][0] = 1;
    for i in 0..n {
        let mut sum = vec![0isize; k + 10];
        for j in 0..=k {
            sum[j + 1] = sum[j] + dp[i][j];
            sum[j + 1] %= MOD;
        }
        for j in 0..=k {
            // 負の整数のmodを取るときは法を足す
            dp[i + 1][j] += sum[j + 1] - sum[max(0, j as isize - a[i] as isize) as usize] + MOD;
            dp[i + 1][j] %= MOD;
        }
    }

    println!("{}", dp[n][k]);
}
