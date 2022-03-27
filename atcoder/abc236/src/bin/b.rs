#![allow(unused_imports)]
#![allow(dead_code)]
use itertools::Itertools;
use proconio::{fastout, input, marker::*};
use std::{cmp::*, collections::*, mem};
const MOD: usize = 1_000_000_007;

#[fastout]
fn main() {
    input! { n: usize, a:[usize; 4 * n - 1] }
    let mut v = vec![0usize; n];
    for i in a {
        v[i - 1] += 1;
    }

    for i in 0..n {
        if v[i] != 4 {
            println!("{}", i + 1);
            return;
        }
    }
}
