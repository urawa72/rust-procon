
use proconio::{fastout, input, marker::Chars};

// 数え上げDP
#[fastout]
fn main() {
    input! { h: usize, w: usize, grid: [Chars; h] }

    let mut dp = vec![vec![0usize; w + 1]; h + 1];

    dp[0][0] = 1;

    for i in 0..h {
        for j in 0..w {
            if i + 1 < h && grid[i + 1][j] == '.' {
                dp[i + 1][j] += dp[i][j];
                dp[i + 1][j] %= 1_000_000_007;
            }
            if j + 1 < w && grid[i][j + 1] == '.' {
                dp[i][j + 1] += dp[i][j];
                dp[i][j + 1] %= 1_000_000_007;
            }
        }
    }
    println!("{}", dp[h - 1][w - 1]);
}
