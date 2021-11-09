#![allow(unused_imports)]
#![allow(dead_code)]
use itertools::Itertools;
use num_integer::Roots;
use proconio::{fastout, input, marker::*};
use std::{cmp::*, collections::*, mem};
const MOD: usize = 1_000_000_007;

#[fastout]
fn main() {
    input! { r: f64, x: f64, y: f64 }
    let d = (x * x + y * y).sqrt();
    if d == r {
        println!("1");
    } else if d <= 2.0 * r {
        println!("2");
    } else {
        println!("{}", (d / r).ceil());
    }
}
