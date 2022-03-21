#![allow(unused_imports)]
#![allow(dead_code)]
use itertools::Itertools;
use proconio::{fastout, input, marker::*};
use std::{cmp::*, collections::*, mem};
const MOD: usize = 1_000_000_007;

#[fastout]
fn main() {
    input! { n: usize, w: usize, mut ab: [(usize, usize); n] }
    ab.sort_by(|a, b| b.0.cmp(&a.0));
    let mut ans = 0;
    let mut sum = 0;
    for i in 0..n {
        for _ in 1..=ab[i].1 {
            if sum + 1 <= w {
                ans += ab[i].0;
                sum += 1;
            }
        }
    }
    println!("{}", ans);
}
