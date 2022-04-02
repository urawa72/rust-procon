#![allow(unused_imports)]
#![allow(dead_code)]
use itertools::Itertools;
use proconio::{fastout, input, marker::*};
use std::{cmp::*, collections::*, mem};
const MOD: usize = 1_000_000_007;

#[fastout]
fn main() {
    input! { n: isize, a: isize, b: isize, c: isize }

    let mut ans = 10000;
    for i in 0..10000 {
        for j in 0..(10000 - i) {
            let tmp = i * a + j * b;
            if 0 <= (n - tmp) && (n - tmp) % c == 0 {
                ans = min(ans, i + j + (n - tmp) / c);
            }
        }
    }
    println!("{}", ans);
}
