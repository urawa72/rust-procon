#![allow(unused_imports)]
#![allow(dead_code)]
use itertools::Itertools;
use proconio::{fastout, input, marker::*};
use std::{cmp::*, collections::*, mem};
const MOD: usize = 1_000_000_007;

#[fastout]
fn main() {
    input! { n: usize, ab: [(usize,usize); n] }

    let mut ans = usize::max_value();

    for i in 0..n {
        for j in 0..n {
            if i == j {
                ans = min(ab[i].0 + ab[i].1, ans);
            } else {
                ans = min(ans, max(ab[i].0, ab[j].1));
            }
        }
    }
    println!("{}", ans);
}
