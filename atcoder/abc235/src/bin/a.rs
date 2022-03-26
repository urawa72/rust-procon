#![allow(unused_imports)]
#![allow(dead_code)]
use itertools::Itertools;
use proconio::{fastout, input, marker::*};
use std::{cmp::*, collections::*, mem};
const MOD: usize = 1_000_000_007;

#[fastout]
fn main() {
    input! { n: usize }
    let a = n / 100;
    let b = n % 100 / 10;
    let c = n % 10;
    let s = a + b + c;
    println!("{}", s * 100 + s * 10 + s);
}
