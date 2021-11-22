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
        let mut mi = a[i];
        for j in i..n {
            mi = min(mi, a[j]);
            ans = max(ans, (j - i + 1) * mi);
        }
    }
    println!("{}", ans);
}
