#![allow(unused_imports)]
#![allow(dead_code)]
use itertools::Itertools;
use proconio::{fastout, input, marker::*};
use std::{cmp::*, collections::*, mem};
const MOD: usize = 1_000_000_007;

#[fastout]
fn main() {
    input! { s: usize, t: usize, x: usize }
    if t < s {
        if s <= x || x < t {
            println!("Yes");
            return;
        }
    } else {
        if s <= x && x < t {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
