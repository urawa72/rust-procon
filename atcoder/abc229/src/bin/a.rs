#![allow(unused_imports)]
#![allow(dead_code)]
use itertools::Itertools;
use proconio::{fastout, input, marker::*};
use std::{cmp::*, collections::*, mem};
const MOD: usize = 1_000_000_007;

#[fastout]
fn main() {
    input! { s1: String, s2: String }

    if (s1 == ".#".to_string() && s2 == "#.".to_string())
        || (s1 == "#.".to_string() && s2 == ".#".to_string())
    {
        println!("No");
    } else {
        println!("Yes");
    }
}
