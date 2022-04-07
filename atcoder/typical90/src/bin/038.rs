#![allow(unused_imports)]
#![allow(dead_code)]
use itertools::Itertools;
use num_integer::gcd;
use proconio::{fastout, input, marker::*};
use std::{cmp::*, collections::*, mem};
const MOD: usize = 1_000_000_007;

#[fastout]
fn main() {
    input! { a: usize, b: usize }
    let p18: usize = 1_000_000_000_000_000_000;
    // lcm(a, b) = a * b / gcm(a, b);
    let x = a / gcd(a, b);
    if p18 / b < x {
        println!("Large");
    } else {
        println!("{}", b * x);
    }
}
