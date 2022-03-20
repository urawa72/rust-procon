#![allow(unused_imports)]
#![allow(dead_code)]
use itertools::Itertools;
use proconio::{fastout, input, marker::*};
use std::{cmp::*, collections::*, mem};
const MOD: usize = 1_000_000_007;

#[fastout]
fn main() {
    input! { n: usize, mut x: usize, a: [usize; n] }
    let mut v = vec![false; n];
    v[x - 1] = true;
    x = a[x - 1];
    loop {
        if v[x - 1] {
            break;
        }
        v[x - 1] = true;
        x = a[x - 1];
    }
    let ans = v.into_iter().filter(|&e| e).count();
    println!("{}", ans);
}
