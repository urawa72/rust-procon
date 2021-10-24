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

    let mut dp = vec![0usize; 1 << n];
    dp[0] = 1;
    for bit in 0..(1 << n) as usize {
        let i = bit.count_ones() as usize;
        for j in 0..n {
            if bit >> j & 1 == 1 && a[i - 1][j] == 1 {
                dp[bit] += dp[bit ^ 1 << j];
                dp[bit] %= MOD;
            }
        }
    }
    println!("{}", dp[(1 << n) - 1]);
}
