#![allow(unused_imports)]
#![allow(dead_code)]
use itertools::Itertools;
use proconio::{fastout, input, marker::*};
use std::{cmp::*, collections::*, mem};
const MOD: usize = 1_000_000_007;

#[fastout]
fn main() {
    input! {
        mut a: i64, mut b: i64, mut k: i64,
    }
    let mut s = String::with_capacity((a + b) as usize);
    while a + b > 0 {
        if a == 0 {
            s.push('b');
            b -= 1;
        } else if b == 0 {
            s.push('a');
            a -= 1;
        } else if comb(a + b - 1, a - 1) < k {
            k -= comb(a - 1 + b, a - 1);
            s.push('b');
            b -= 1;
        } else {
            s.push('a');
            a -= 1;
        }
    }
    println!("{}", s);
}

fn comb(n: i64, r: i64) -> i64 {
    let mut v = 1;
    for i in 1..=r {
        v = v * (n + 1 - i) / i;
    }
    v
}
