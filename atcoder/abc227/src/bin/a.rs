#![allow(unused_imports)]
#![allow(dead_code)]
use itertools::Itertools;
use proconio::{fastout, input, marker::*};
use std::{cmp::*, collections::*, mem};
const MOD: usize = 1_000_000_007;

#[fastout]
fn main() {
    input! { n: usize, mut k: usize, a: usize };
    let mut now = a;
    while 1 < k {
        k = k - 1;
        if now == n {
            now = 1;
        } else {
            now = now + 1;
        }
    }
    println!("{}", now);
}
