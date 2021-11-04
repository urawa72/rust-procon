#![allow(unused_imports)]
#![allow(dead_code)]
use itertools::Itertools;
use proconio::{fastout, input, marker::*};
use std::{cmp::*, collections::*, mem};
const MOD: usize = 1_000_000_007;

#[fastout]
fn main() {
    input! { s: Chars }
    let mut ans = 0;
    for mut i in 0..=9999 {
        let mut f = vec![false; 10];
        for _ in 0..4 {
            f[i % 10] = true;
            i /= 10;
        }
        let mut p = 1;
        for j in 0..10 {
            if s[j] == 'o' && !f[j] {
                p = 0;
            }
            if s[j] == 'x' && f[j] {
                p = 0;
            }
        }
        ans += p;
    }
    println!("{}", ans);
}
