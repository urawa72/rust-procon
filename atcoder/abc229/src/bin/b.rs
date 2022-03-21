#![allow(unused_imports)]
#![allow(dead_code)]
use itertools::Itertools;
use proconio::{fastout, input, marker::*};
use std::{cmp::*, collections::*, mem};
const MOD: usize = 1_000_000_007;

#[fastout]
fn main() {
    input! { mut a: usize, mut b: usize }

    while 0 < a || 0 < b {
        let x = a % 10;
        let y = b % 10;
        if 10 <= x + y {
            println!("Hard");
            return;
        }
        a /= 10;
        b /= 10;
    }
    println!("Easy");
}
