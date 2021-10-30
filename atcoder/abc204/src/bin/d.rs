#![allow(unused_imports)]
#![allow(dead_code)]
use itertools::Itertools;
use proconio::{fastout, input, marker::*};
use std::{cmp::*, collections::*, mem};
const MOD: usize = 1_000_000_007;

#[fastout]
fn main() {
    input! { n: usize, t: [usize; n] }

    let half = t.iter().sum::<usize>() / 2;
    let mut dp = vec![vec![0usize; half + 1]; n + 1];

    for i in 0..n {
        for j in 0..=half {
            if t[i] <= j {
                dp[i + 1][j] = (dp[i][j - t[i]] + t[i]).max(dp[i][j]);
            } else {
                dp[i + 1][j] = dp[i][j];
            }
        }
    }
    println!("{}", t.iter().sum::<usize>() - dp[n][half]);
}
