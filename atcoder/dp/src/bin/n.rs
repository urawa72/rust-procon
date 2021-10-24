use proconio::{fastout, input};
use std::cmp::min;

// 区間DP
#[fastout]
fn main() {
    input! { n: usize, a:[usize; n] }

    // 累積和
    let mut sum = vec![0usize; n + 1];
    for i in 0..n {
        sum[i + 1] = sum[i] + a[i];
    }

    let mut dp = vec![vec![0usize; n + 1]; n + 1];
    println!("{}", rec(0, n, &mut dp, &sum));
}

fn rec(l: usize, r: usize, dp: &mut Vec<Vec<usize>>, sum: &Vec<usize>) -> usize {
    if l + 1 == r {
        return 0;
    }

    if 0 < dp[l][r] {
        return dp[l][r];
    }

    let mut res = 1 << 60;
    for i in l + 1..r {
        res = min(res, rec(l, i, dp, sum) + rec(i, r, dp, sum));
    }
    dp[l][r] = res + sum[r] - sum[l];
    dp[l][r]
}
