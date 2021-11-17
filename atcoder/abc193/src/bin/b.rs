#![allow(unused_imports)]
#![allow(dead_code)]
use itertools::Itertools;
use proconio::{fastout, input, marker::*};
use std::{cmp::*, collections::*, mem};
const MOD: usize = 1_000_000_007;

#[fastout]
fn main() {
    input! { n: usize, t: [(isize, usize, isize); n] }

    let mut ans = usize::max_value();
    let mut c: isize = 0;
    for i in 0..n {
        c += t[i].0 - c;
        if 0 < t[i].2 - c {
            ans = min(ans, t[i].1);
        }
    }

    if ans != usize::max_value() {
        println!("{}", ans);
    } else {
        println!("-1");
    }
}
