#![allow(unused_imports)]
#![allow(dead_code)]
use itertools::Itertools;
use num_integer::Roots;
use proconio::{fastout, input, marker::*};
use std::{cmp::*, collections::*, mem};
const MOD: usize = 1_000_000_007;

#[fastout]
fn main() {
    input! { n: usize, p: [(f64, f64); n] }
    let mut ans = 0.0;
    for i in 0..n {
        for j in i..n {
            let t = ((p[j].0 - p[i].0).powf(2.0) + (p[j].1 - p[i].1).powf(2.0)).sqrt();
            if ans < t {
                ans = t;
            }
        }
    }
    println!("{}", ans);
}
