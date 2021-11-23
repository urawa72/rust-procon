#![allow(unused_imports)]
#![allow(dead_code)]
use itertools::Itertools;
use petgraph::visit::Walker;
use proconio::{fastout, input, marker::*};
use std::{cmp::*, collections::*, mem};
const MOD: usize = 1_000_000_007;

fn main() {
    input! { n: usize, a: [isize; n], b: [isize; n] }

    let ans: isize = a
        .iter()
        .zip(b.iter())
        .map(|(x, y)| x * y)
        .collect::<Vec<_>>()
        .iter()
        .sum();
    println!("{}", if ans == 0 { "Yes" } else { "No" });
}
