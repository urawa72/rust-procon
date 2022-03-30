#![allow(unused_imports)]
#![allow(dead_code)]
use itertools::Itertools;
use proconio::{fastout, input, marker::*};
use std::{cmp::*, collections::*, mem};
const MOD: usize = 1_000_000_007;

#[fastout]
fn main() {
    input! { n: usize }
    let mut map = HashMap::new();
    for i in 1..=n {
        input! { s: String }
        let e = map.entry(s).or_insert(0);
        *e += 1;
        if *e == 1 {
            println!("{}", i);
        }
    }
}
