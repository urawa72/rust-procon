#![allow(unused_imports)]
#![allow(dead_code)]
use itertools::Itertools;
use proconio::{fastout, input, marker::*};
use std::{cmp::*, collections::*, mem};
const MOD: usize = 1_000_000_007;

#[fastout]
fn main() {
    input! { a: isize, b: isize, c: usize }

    if c % 2 == 0 {
        if a.abs() < b.abs() {
            println!("<");
        }
        if a.abs() > b.abs() {
            println!(">");
        }
        if a.abs() == b.abs() {
            println!("=");
        }
    } else {
        if a < b {
            println!("<");
        }
        if a > b {
            println!(">");
        }
        if a == b {
            println!("=");
        }
    }
}
