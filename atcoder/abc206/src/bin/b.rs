#![allow(unused_imports)]
#![allow(dead_code)]
use itertools::Itertools;
use proconio::{fastout, input, marker::*};
use std::{cmp::*, collections::*, mem};
const MOD: usize = 1_000_000_007;

#[fastout]
fn main() {
    input! { n: usize }
    let mut sum = 0;
    let mut c = 0;
    while sum < n {
        c += 1;
        sum += c;
    }
    println!("{}", c);
}
