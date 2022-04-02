#![allow(unused_imports)]
#![allow(dead_code)]
use itertools::Itertools;
use proconio::{fastout, input, marker::*};
use std::{cmp::*, collections::*, mem};
const MOD: usize = 1_000_000_007;

#[fastout]
fn main() {
    input! { n: usize, m: usize, ab: [(usize, usize); m] }

    let mut g = vec![vec![]; n];
    for i in 0..m {
        g[ab[i].0 - 1].push(ab[i].1 - 1);
        g[ab[i].1 - 1].push(ab[i].0 - 1);
    }

    let mut ans = 0;
    for i in 0..n {
        g[i].sort();
        if 1 < g[i].len() && g[i][0] < i && g[i][1] > i {
            ans += 1;
        }
        if g[i].len() == 1 && g[i][0] < i {
            ans += 1;
        }
    }
    println!("{}", ans);
}
