#![allow(unused_imports)]
#![allow(dead_code)]
use itertools::Itertools;
use proconio::{fastout, input, marker::*};
use std::{cmp::*, collections::*, mem};
const MOD: usize = 1_000_000_007;

#[fastout]
fn main() {
    input! { n: usize, mut a: [isize; n], mut b: [isize; n] }

    a.sort();
    b.sort();

    let mut ans = 0;
    for i in 0..n {
        ans += (a[i] - b[i]).abs();
    }
    println!("{}", ans);
}
