#![allow(unused_imports)]
#![allow(dead_code)]
use itertools::Itertools;
use proconio::{fastout, input, marker::*};
use std::{cmp::*, collections::*, mem};
const MOD: usize = 1_000_000_007;

#[fastout]
fn main() {
    input! { a: usize, b: usize, mut w: usize }
    w *= 1000;

    let mut mi = usize::max_value();
    let mut ma = 0;
    for i in 1..=w {
        let l = i * a;
        let r = i * b;
        if l <= w && w <= r  {
            mi = min(mi, i);
            ma = max(ma, i);
        }
    }

    if mi == usize::max_value() {
        println!("UNSATISFIABLE");
    } else {
        println!("{} {}", mi, ma);
    }

}
