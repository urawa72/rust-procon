#![allow(unused_imports)]
#![allow(dead_code)]
use itertools::Itertools;
use proconio::{fastout, input, marker::*};
use std::{cmp::*, collections::*, mem};
const MOD: usize = 1_000_000_007;

#[fastout]
fn main() {
    input! { n: usize, x: usize, a: [usize; n] }

    for i in 0..n {
        if a[i] == x {
            continue;
        } else {
            if i != n - 1 {
                print!("{} ", a[i]);
            } else {
                println!("{}", a[i]);
            }
        }
    }
}
