#![allow(unused_imports)]
#![allow(dead_code)]
use itertools::Itertools;
use proconio::{fastout, input, marker::*};
use std::{cmp::*, collections::*, mem};
const MOD: usize = 1_000_000_007;

#[fastout]
fn main() {
    input! { n: usize, s: usize, d: usize, xy: [(usize, usize); n] }

    let c = xy.iter().filter(|e| e.0 < s && d < e.1).count();
    if 0 < c {
        println!("Yes");
    } else {
        println!("No");
    }
}
