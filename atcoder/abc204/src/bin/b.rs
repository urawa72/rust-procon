#![allow(unused_imports)]
#![allow(dead_code)]
use itertools::Itertools;
use proconio::{fastout, input, marker::*};
use std::{cmp::*, collections::*, mem};
const MOD: usize = 1_000_000_007;

#[fastout]
fn main() {
    input! { n: usize, a: [usize; n] }
    let mut ans = 0;
    for i in 0..n {
        if 10 < a[i] {
            ans += a[i] - 10;
        }
    }
    println!("{}", ans);
}
