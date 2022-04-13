#![allow(unused_imports)]
#![allow(dead_code)]
use itertools::Itertools;
use proconio::{fastout, input, marker::*};
use std::{cmp::*, collections::*, mem};
const MOD: usize = 1_000_000_007;

#[fastout]
fn main() {
    input! { n: usize, l: usize }

    let mut dp = vec![0usize; n + 1];

    dp[0] = 1;
    for i in 0..n {
        dp[i + 1] += dp[i];
        dp[i + 1] %= MOD;
        if i + l <= n {
            dp[i + l] += dp[i];
            dp[i + l] %= MOD;
        }
    }
    println!("{}", dp[n] % MOD);
}
