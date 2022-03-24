#![allow(unused_imports)]
#![allow(dead_code)]
use itertools::Itertools;
use proconio::{fastout, input, marker::*};
use std::{cmp::*, collections::*, mem};
const MOD: usize = 1_000_000_007;

#[fastout]
fn main() {
    input! { t: usize }
    let f = |e: usize| e * e + 2 * e + 3;
    println!("{}", f(f(f(t) + t) + f(f(t))));
}
