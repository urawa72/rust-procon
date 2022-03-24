#![allow(unused_imports)]
#![allow(dead_code)]
use itertools::Itertools;
use proconio::{fastout, input, marker::*};
use std::{cmp::*, collections::*, mem};
const MOD: usize = 1_000_000_007;

#[fastout]
fn main() {
    input! { mut k: usize }
    let mut v = vec![];
    while 1 < k {
        v.push(k % 2);
        k /= 2;
    }
    if k != 0 {
        v.push(k);
    }

    v.reverse();
    for i in v {
        print!("{}", if i == 1 { 2 } else { 0 });
    }
    println!()
}
