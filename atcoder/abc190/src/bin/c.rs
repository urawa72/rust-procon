#![allow(unused_imports)]
#![allow(dead_code)]
use itertools::Itertools;
use proconio::{fastout, input, marker::*};
use std::{cmp::*, collections::*, mem};
const MOD: usize = 1_000_000_007;

#[fastout]
fn main() {
    input! { n: usize, m: usize, ab: [(usize, usize); m], k: usize, cd: [(usize, usize); k] }

    let mut ans = 0;
    for bit in 0..1 << k {
        let mut v = vec![0usize; n + 1];
        for i in 0..k {
            if bit & 1 << i != 0 {
                v[cd[i].0] += 1;
            } else {
                v[cd[i].1] += 1;
            }
        }
        let mut cnt = 0;
        for i in 0..m {
            if 0 < v[ab[i].0] && 0 < v[ab[i].1] {
                cnt += 1;
            }
        }
        ans = max(ans, cnt);
    }
    println!("{}", ans);
}
