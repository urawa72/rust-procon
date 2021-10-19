use proconio::{fastout, input};

// 確率DP
#[fastout]
fn main() {
    input! { n: usize, p: [f64; n] }

    let mut dp = vec![vec![0f64; n + 10]; n + 10];

    dp[0][0] = 1.0;
    // 最初のi枚のコインを投げたときに表がj枚になる確率
    for i in 0..n {
        for j in 0..=i {
            dp[i + 1][j + 1] += dp[i][j] * p[i];
            dp[i + 1][j] += dp[i][j] * (1.0 - p[i]);
        }
    }

    let res: f64 = (n / 2 + 1..=n)
        .map(|i| dp[n][i])
        .fold(0.0, |sum, j| sum + j);
    println!("{}", res);
}
