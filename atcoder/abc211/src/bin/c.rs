use itertools::Itertools;
use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! { s: Chars }
    let n = s.len();
    let mut dp = vec![vec![0; 9]; n + 1];
    dp[0][0] = 1;
    let m = 1000000007;
    let t = "chokudai".chars().collect_vec();
    for i in 0..n {
        for j in 0..9 {
            dp[i + 1][j] += dp[i][j];
            dp[i + 1][j] %= m;
            for k in 0..8 {
                if s[i] == t[k] && j == k {
                    dp[i + 1][j + 1] += dp[i][j];
                    dp[i + 1][j + 1] %= m;
                }
            }
        }
    }
    println!("{}", dp[n][8]);
}
