#![allow(unused_imports)]
#![allow(dead_code)]
use itertools::Itertools;
use proconio::{fastout, input, marker::*};
use std::{cmp::*, collections::*, mem};
const MOD: usize = 1_000_000_007;

#[fastout]
fn main() {
    input! { a: Chars, b: Chars }

    let a = a
        .iter()
        .map(|&c| c as i32 - 48)
        .collect::<Vec<_>>()
        .iter()
        .sum::<i32>();
    let b= b
        .iter()
        .map(|&c| c as i32 - 48)
        .collect::<Vec<_>>()
        .iter()
        .sum::<i32>();

    println!("{}", max(a, b));
}
