#![allow(unused_imports)]
#![allow(dead_code)]
use itertools::Itertools;
use proconio::{fastout, input, marker::*};
use std::{cmp::*, collections::*, mem};
const MOD: usize = 1_000_000_007;

#[fastout]
fn main() {
    input! { h: usize, w: usize, mut a: [[isize; w]; h], b: [[isize; w]; h] }

    let mut ans = 0;
    for i in 0..h - 1 {
        for j in 0..w - 1 {
            let d = b[i][j] - a[i][j];
            a[i][j] += d;
            a[i][j + 1] += d;
            a[i + 1][j] += d;
            a[i + 1][j + 1] += d;
            ans += d.abs();
        }
    }
    if a == b {
        println!("Yes");
        println!("{}", ans);
    } else {
        println!("No");
    }
}
