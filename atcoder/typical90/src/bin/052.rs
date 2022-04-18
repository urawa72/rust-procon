#![allow(unused_imports)]
#![allow(dead_code)]
use itertools::Itertools;
use proconio::{fastout, input, marker::*};
use std::{cmp::*, collections::*, mem};
const MOD: usize = 1_000_000_007;

#[fastout]
fn main() {
    input! { n: usize, a: [[usize; 6]; n] }

    let mut ans = 1;
    for i in 0..n {
        let x = a[i][0] + a[i][1] + a[i][2] + a[i][3] + a[i][4] + a[i][5];
        ans *= x;
        ans %= MOD;
    }

    println!("{}", ans);
}
