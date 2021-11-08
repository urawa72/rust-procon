#![allow(unused_imports)]
#![allow(dead_code)]
use itertools::{concat, Itertools};
use proconio::{fastout, input, marker::*};
use std::{cmp::*, collections::*, mem};
const MOD: usize = 1_000_000_007;

#[fastout]
fn main() {
    input! {n: usize, ab: [(usize, usize); n - 1]}
    let mut map = HashMap::new();
    for i in 0..n - 1 {
        *map.entry(ab[i].0).or_insert(0) += 1;
        *map.entry(ab[i].1).or_insert(0) += 1;
    }
    for i in 1..=n {
        let e = *map.entry(i).or_insert(0);
        if e == 1 {
            continue;
        } else if e == n - 1 {
            continue;
        } else {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
