#![allow(unused_imports)]
#![allow(dead_code)]
use itertools::Itertools;
use proconio::{fastout, input, marker::*};
use std::{cmp::*, collections::*, mem};
const MOD: usize = 1_000_000_007;

#[fastout]
fn main() {
    input! { n: usize, h: [usize; n] }
    let mut ans = h[0];
    for i in 0..n - 1 {
        if h[i] < h[i + 1] {
            ans = h[i + 1];
        } else {
            break;
        }
    }
    println!("{}", ans);
}
