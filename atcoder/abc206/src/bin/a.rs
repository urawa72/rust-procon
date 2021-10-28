#![allow(unused_imports)]
#![allow(dead_code)]
use itertools::Itertools;
use proconio::{fastout, input, marker::*};
use std::{cmp::*, collections::*, mem};
const MOD: usize = 1_000_000_007;

#[fastout]
fn main() {
    input! { x: f64 }
    let x = (x * 1.08).floor() as usize;
    if x < 206 {
        println!("Yay!");
    } else if x == 206 {
        println!("so-so");
    } else {
        println!(":(");
    }
}
