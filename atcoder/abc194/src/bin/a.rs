#![allow(unused_imports)]
#![allow(dead_code)]
use itertools::Itertools;
use proconio::{fastout, input, marker::*};
use std::{cmp::*, collections::*, mem};
const MOD: usize = 1_000_000_007;

#[fastout]
fn main() {
    input! { a: usize, b: usize }
    if 15 <= a + b && 8 <= b {
        println!("1");
    } else if 10 <= a + b && 3 <= b {
        println!("2");
    } else if 3 <= a + b {
        println!("3");
    } else {
        println!("4");
    }
}
