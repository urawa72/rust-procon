#![allow(unused_imports)]
#![allow(dead_code)]
use itertools::Itertools;
use proconio::{fastout, input, marker::*};
use std::{cmp::*, collections::*, mem};
const MOD: usize = 1_000_000_007;

#[fastout]
fn main() {
    input! { mut n: Chars, k: usize }

    for _ in 0..k {
        n.sort();
        let t: String = n.iter().collect();
        n.reverse();
        let r: String = n.iter().collect();
        let a: usize = t.parse().unwrap();
        let b: usize = r.parse().unwrap();
        n = (b - a).to_string().chars().collect();
    }

    println!("{}", n.iter().collect::<String>());
}
