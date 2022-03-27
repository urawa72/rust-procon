#![allow(unused_imports)]
#![allow(dead_code)]
use itertools::Itertools;
use proconio::{fastout, input, marker::*};
use std::{cmp::*, collections::*, mem};
const MOD: usize = 1_000_000_007;

#[fastout]
fn main() {
    input! { s: Chars, a: usize, b: usize }

    for i in 0..s.len() {
        if i == a - 1 {
            print!("{}", s[b - 1]);
        } else if i == b - 1 {
            print!("{}", s[a - 1]);
        } else {
            print!("{}", s[i]);
        }
    }
    println!()

}
