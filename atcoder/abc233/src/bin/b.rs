#![allow(unused_imports)]
#![allow(dead_code)]
use itertools::Itertools;
use proconio::{fastout, input, marker::*};
use std::{cmp::*, collections::*, mem};
const MOD: usize = 1_000_000_007;

#[fastout]
fn main() {
    input! { l: usize, r: usize, s: String }

    let s = s.chars().collect::<Vec<_>>();
    let mut c = r;
    for i in 0..s.len() {
        if l - 1 <= i && i < r {
            print!("{}", s[c - 1]);
            c -= 1;
        } else {
            print!("{}", s[i]);
        }
    }
    println!();
}
