#![allow(unused_imports)]
#![allow(dead_code)]
use itertools::Itertools;
use num_integer::gcd;
use proconio::{fastout, input, marker::*};
use std::{cmp::*, collections::*, mem};
const MOD: usize = 1_000_000_007;

#[fastout]
fn main() {
    input! { a: usize, b: usize, c: usize }

    let g = gcd(c, gcd(a, b));
    let ans = (a / g - 1) + (b / g - 1) + (c / g - 1);
    println!("{}", ans);

}

