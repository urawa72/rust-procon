use proconio::{input, fastout};
use std::cmp::max;

#[fastout]
fn main() {
    input! { n: usize, m: usize, xy: [(usize, usize); m] }
    let mut g = vec![Vec::new(); n];
    for (mut x, mut y) in xy {
        x -= 1;
        y -= 1;
        g[x].push(y);
    }

    let mut dp = vec![-1; n];
    let mut res = 0;
    for v in 0..n {
        res = max(res, rec(v, &g, &mut dp));
    }
    println!("{}", res);
}

fn rec(v: usize, g: &[Vec<usize>], dp: &mut [isize]) -> isize {
    if 0 <= dp[v] {
        return dp[v];
    }

    let mut res = 0;
    for &nv in &g[v] {
        res = max(res, rec(nv, g, dp) + 1);
    }
    dp[v] = res;
    res
}
