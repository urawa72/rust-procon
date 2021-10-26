#![allow(unused_imports)]
#![allow(dead_code)]
use itertools::Itertools;
use proconio::{fastout, input, marker::*};
use std::{cmp::*, collections::*, mem};

#[fastout]
fn main() {
    input! { n: usize, v: [(usize, f64, f64); n] }

    let ans = v
        .iter()
        .map(|&(t, l, r)| match t {
            1 => (l, r),
            2 => (l, r - 0.5),
            3 => (l + 0.4, r),
            4 => (l + 0.5, r - 0.5),
            _ => (l, r),
        })
        .tuple_combinations()
        .filter(|&((l1, r1), (l2, r2))| l1.max(l2) <= r1.min(r2))
        .count();
    println!("{}", ans);
}
