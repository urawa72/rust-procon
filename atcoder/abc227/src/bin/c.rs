#![allow(unused_imports)]
#![allow(dead_code)]
use itertools::Itertools;
use proconio::{fastout, input, marker::*};
use std::{cmp::*, collections::*, mem};
const MOD: usize = 1_000_000_007;

#[fastout]
fn main() {
    input! { n: usize }

    let mut ans = 0;
    for a in 1..=n {
        if a * a * a > n {
            break;
        }
        for b in a..=n {
            if a * b * b > n {
                break;
            }
            let c = n / (a * b);
            if b <= c {
                ans += c - b + 1;
            }
        }
    }
    println!("{}", ans);
}
