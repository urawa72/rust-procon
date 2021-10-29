#![allow(unused_imports)]
#![allow(dead_code)]
use itertools::Itertools;
use proconio::{fastout, input, marker::*};
use std::{cmp::*, collections::*, mem};
const MOD: usize = 1_000_000_007;

#[fastout]
fn main() {
    input! { n: usize, a: [usize; n] }

    let mut c = vec![0usize; n + 1];
    for &e in a.iter() {
        c[e] += 1;
    }
    let ans = c.iter().filter(|&m| 1 < *m).count();
    println!("{}", if ans == 0 { "Yes" } else { "No" });
}
