#![allow(unused_imports)]
#![allow(dead_code)]
use itertools::Itertools;
use proconio::{fastout, input, marker::*};
use std::{cmp::*, collections::*, mem};
const MOD: usize = 1_000_000_007;

#[fastout]
fn main() {
    input! { n: usize, mut s: [(String, usize); n] }
    s.sort_by_key(|k| k.1);
    println!("{}", s[n - 2].0);

}
