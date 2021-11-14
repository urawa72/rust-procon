#![allow(unused_imports)]
#![allow(dead_code)]
use itertools::Itertools;
use proconio::{fastout, input, marker::*};
use std::{cmp::*, collections::*, mem};
const MOD: usize = 1_000_000_007;

#[fastout]
fn main() {
    input! { n: usize }
    let mut set = HashSet::new();
    for _ in 0..n {
        input! { l: usize, a: [usize; l] }
        set.insert(a);
    }
    println!("{}", set.len());
}
