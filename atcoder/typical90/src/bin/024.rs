#![allow(unused_imports)]
#![allow(dead_code)]
use itertools::Itertools;
use proconio::{fastout, input, marker::*};
use std::{cmp::*, collections::*, mem};
const MOD: usize = 1_000_000_007;

#[fastout]
fn main() {
    input! { n: usize, k: isize, a: [isize; n], b: [isize; n] }

    let mut cnt = 0;
    for i in 0..n {
        cnt += (a[i] - b[i]).abs();
    }

    if 0 <= (k - cnt) && (k - cnt) % 2 == 0 {
        println!("Yes");
    } else {
        println!("No");
    }
}
