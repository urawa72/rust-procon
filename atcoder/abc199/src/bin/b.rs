#![allow(unused_imports)]
#![allow(dead_code)]
use itertools::Itertools;
use proconio::{fastout, input, marker::*};
use std::{cmp::*, collections::*, mem};
const MOD: usize = 1_000_000_007;

#[fastout]
fn main() {
    input! { n: usize, a: [usize; n], b: [usize; n] }
    let x = a.iter().max().unwrap();
    let y = b.iter().min().unwrap();
    if y < x {
        println!("{}", 0);
    } else {
        println!("{}", y - x + 1);
    }
}
