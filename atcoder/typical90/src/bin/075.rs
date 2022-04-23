#![allow(unused_imports)]
#![allow(dead_code)]
use itertools::Itertools;
use proconio::{fastout, input, marker::*};
use std::{cmp::*, collections::*, mem};
const MOD: usize = 1_000_000_007;

#[fastout]
fn main() {
    input! { n: usize }

    let v = prime_factors(n);
    for i in 0..=20 {
        if v.len() <= 1 << i {
            println!("{}", i);
            return;
        }
    }
}

fn prime_factors(n: usize) -> Vec<usize> {
    let mut rem = n;
    let mut v = vec![];
    for i in 2..=n {
        if i * i > n {
            break;
        }
        // 指数
        // let mut e = 0;
        while rem % i == 0 {
            rem /= i;
            v.push(i);
            // e += 1;
        }
    }
    if rem != 1 {
        v.push(rem);
    }
    v
}
