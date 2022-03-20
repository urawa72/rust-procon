#![allow(unused_imports)]
#![allow(dead_code)]
use itertools::Itertools;
use proconio::{fastout, input, marker::*};
use std::{cmp::*, collections::*, mem};
const MOD: usize = 1_000_000_007;

#[fastout]
fn main() {
    input! {n: usize, k: usize, p: [[usize; 3]; n]}
    let v: Vec<usize> = p.iter().map(|v| v.iter().sum()).sorted().rev().take(k).collect();
    let w: Vec<usize> = p.iter().map(|v| v.iter().sum()).collect();
    for e in w {
        if v[k - 1] <= e + 300 {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
