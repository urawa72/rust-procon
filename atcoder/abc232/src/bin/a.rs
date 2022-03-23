#![allow(unused_imports)]
#![allow(dead_code)]
use itertools::Itertools;
use proconio::{fastout, input, marker::*};
use std::{cmp::*, collections::*, mem};
const MOD: usize = 1_000_000_007;

#[fastout]
fn main() {
    input! { s: String }
    let v = s.chars().collect::<Vec<_>>();
    println!("{}", (v[0] as i32 - 48) * (v[2] as i32 - 48));
}
