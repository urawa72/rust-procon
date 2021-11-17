#![allow(unused_imports)]
#![allow(dead_code)]
use itertools::Itertools;
use proconio::{fastout, input, marker::*};
use std::{cmp::*, collections::*, mem};
const MOD: usize = 1_000_000_007;

#[fastout]
fn main() {
    input! { n: usize }

    let mut s = HashSet::new();
    for i in 2..=100_000 {
        let mut tmp = i * i;
        while tmp <= n {
            s.insert(tmp);
            tmp *= i;
        }
    }
    println!("{}", n - s.len());
}
