#![allow(unused_imports)]
#![allow(dead_code)]
use itertools::Itertools;
use proconio::{fastout, input, marker::*};
use std::{cmp::*, collections::*, mem};
const MOD: usize = 1_000_000_007;

#[fastout]
fn main() {
    input! { s: String }

    let mut t = "".to_string();
    for _ in 0..10 {
        t = t + "oxx";
    }
    if t.contains(&s) {
        println!("Yes");
    } else {
        println!("No");
    }
}
