#![allow(unused_imports)]
#![allow(dead_code)]
use itertools::Itertools;
use proconio::{fastout, input, marker::*};
use std::{cmp::*, collections::*, mem};
const MOD: usize = 1_000_000_007;

#[fastout]
fn main() {
    input! { a: usize, b: usize, c: usize }

    if c == 0 {
        if a <= b {
            println!("Aoki");
        } else {
            println!("Takahashi");
        }
    } else {
        if a < b {
            println!("Aoki");
        } else {
            println!("Takahashi");
        }
    }
}
