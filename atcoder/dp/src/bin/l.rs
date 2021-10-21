use proconio::{fastout, input};

// 区間DP
#[fastout]
fn main() {
    input! { n: usize, a: [isize; n] }

    let mut dp = vec![vec![0isize; n + 10]; n + 10];

    for s in 1..=n {
        for r in s..=n {
            let l = r - s;
            if (n - (r - l)) % 2 == 0 {
                dp[l][r] = std::cmp::max(dp[l][r - 1] + a[r - 1], dp[l + 1][r] + a[l]);
            } else {
                dp[l][r] = std::cmp::min(dp[l][r - 1] - a[r - 1], dp[l + 1][r] - a[l]);
            }
        }
    }

    println!("{}", dp[0][n]);
}
