#![allow(unused_imports)]
use itertools::Itertools;
use ordered_float::OrderedFloat;
use proconio::{fastout, input, marker::*};
use std::{cmp::*, collections::*, mem};
use superslice::*;
const MOD: usize = 1_000_000_007;

// bitDP
#[fastout]
fn main() {
    input! {
        n: usize,
        a: [[usize; n]; n]
    }

    let mut dp = vec![vec![0usize; 1 << n]; n + 1];
    dp[0][0] = 1;
    for i in 0..n {
        for s in 0..(1 << n) as usize {
            let c = s.count_ones() as usize;
            if i == c {
                for j in 0..n {
                    if s & 1 << j != 1 && a[i][j] == 1 {
                        dp[i + 1][s | (1 << j)] += dp[i][s];
                        dp[i + 1][s | (1 << j)] %= MOD;
                    }
                }
            }

        }
    }
    println!("{}", dp[n][(1 << n) - 1]);
}
