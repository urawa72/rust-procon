#![allow(unused_imports)]
#![allow(dead_code)]
use itertools::Itertools;
use proconio::{fastout, input, marker::*};
use std::{cmp::*, collections::*, mem};
const MOD: usize = 1_000_000_007;

#[fastout]
fn main() {
    input! { n: usize, mut s: [isize; n] }

    let mut cnt = 0;
    for a in 1..=1000 {
        for b in 1..=1000 {
            for i in 0..n {
                let tmp = 4 * a * b + 3 * a + 3 * b;
                if s[i] != -1 && tmp == s[i] {
                    cnt = cnt + 1;
                    s[i] = -1;
                }
            }
        }
    }
    println!("{}", n - cnt);
}
