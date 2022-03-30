#![allow(unused_imports)]
#![allow(dead_code)]
use itertools::Itertools;
use proconio::{fastout, input, marker::*};
use std::{cmp::*, collections::*, mem};
const MOD: usize = 1_000_000_007;

#[fastout]
fn main() {
    input! { h: usize, w: usize }
    if h == 1 || w == 1 {
        println!("{}", max(h, w));
    } else {
        println!("{}", ((h + 1) / 2) * ((w + 1) / 2));
    }
}
