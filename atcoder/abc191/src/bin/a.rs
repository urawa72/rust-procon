#![allow(unused_imports)]
#![allow(dead_code)]
use itertools::Itertools;
use proconio::{fastout, input, marker::*};
use std::{cmp::*, collections::*, mem};
const MOD: usize = 1_000_000_007;

#[fastout]
fn main() {
    input! { v: f64, t: f64, s: f64, d: f64 }

    println!(
        "{}",
        if t <= d / v && d / v <= s {
            "No"
        } else {
            "Yes"
        }
    );
}
