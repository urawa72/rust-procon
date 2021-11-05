#![allow(unused_imports)]
#![allow(dead_code)]
use itertools::Itertools;
use proconio::{fastout, input, marker::*};
use std::{cmp::*, collections::*, mem};
const MOD: usize = 1_000_000_007;

#[fastout]
fn main() {
    input! { mut n: usize, k: usize }

    for _ in 0..k {
        if n % 200 == 0 {
            n /= 200;
        } else {
            let mut s = n.to_string();
            s += "200";
            n = s.parse::<usize>().unwrap();
        }
    }
    println!("{}", n);
}
