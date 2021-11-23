#![allow(unused_imports)]
#![allow(dead_code)]
use itertools::Itertools;
use proconio::{fastout, input, marker::*};
use std::{cmp::*, collections::*, mem};
const MOD: usize = 1_000_000_007;

fn main() {
    input! { x: usize, y: usize }
    println!(
        "{}",
        if max(x, y) < min(x, y) + 3 {
            "Yes"
        } else {
            "No"
        }
    );
}
