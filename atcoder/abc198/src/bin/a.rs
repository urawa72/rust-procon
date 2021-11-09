#![allow(unused_imports)]
#![allow(dead_code)]
use itertools::Itertools;
use proconio::{fastout, input, marker::*};
use std::{cmp::*, collections::*, mem};
const MOD: usize = 1_000_000_007;

#[fastout]
fn main() {
    input! { n: usize }

    let mut ans = 0;
    for i in 1..=n / 2 {
        let m = n - i;
        if i == m {
            ans += 1;
        } else {
            ans += 2;
        }
    }
    println!("{}", ans);
}
