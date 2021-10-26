#![allow(unused_imports)]
#![allow(dead_code)]
use itertools::Itertools;
use proconio::{fastout, input, marker::*};
use std::{cmp::*, collections::*, mem};
const MOD: usize = 1_000_000_007;

#[fastout]
fn main() {
    input! { mut a: usize, b: usize, c: usize, d: usize }

    if c * d <= b {
        println!("{}", -1);
        return;
    }

    let mut ans = 0;
    let mut tmp = 0;
    while tmp * d < a {
        a += b;
        tmp += c;
        ans += 1;
    }

    println!("{}", ans);
}
