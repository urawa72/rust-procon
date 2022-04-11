#![allow(unused_imports)]
#![allow(dead_code)]
use itertools::Itertools;
use proconio::{fastout, input, marker::*};
use std::{cmp::*, collections::*, mem};
const MOD: usize = 1_000_000_007;

#[fastout]
fn main() {
    input! { n: usize, k: usize, ab: [(usize, usize); n] }

    // bはa/2より必ず小さいので、
    // bとa-bをソートして大きい順に並べ先頭からk取ればそれが答え
    let mut v = vec![];
    for (a, b) in ab {
        v.push(b);
        v.push(a - b);
    }

    let mut ans = 0;
    v.sort();
    v.reverse();
    for i in 0..k {
        ans += v[i];
    }
    println!("{}", ans);
}
