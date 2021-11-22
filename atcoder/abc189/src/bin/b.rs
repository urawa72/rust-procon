#![allow(unused_imports)]
#![allow(dead_code)]
use itertools::Itertools;
use proconio::{fastout, input, marker::*};
use std::{cmp::*, collections::*, mem};
const MOD: usize = 1_000_000_007;

#[fastout]
fn main() {
    input! { n: usize, mut x: usize, vp: [(usize, usize); n] }

    x *= 100;
    let mut sum = 0;
    for i in 0..n {
        sum += vp[i].0 * vp[i].1;
        if x < sum {
            println!("{}", i + 1);
            return;
        }
    }
    println!("-1");
}
