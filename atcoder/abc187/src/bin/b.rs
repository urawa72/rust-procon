#![allow(unused_imports)]
#![allow(dead_code)]
use itertools::Itertools;
use proconio::{fastout, input, marker::*};
use std::{cmp::*, collections::*, mem};
const MOD: usize = 1_000_000_007;

#[fastout]
fn main() {
    input! { n: usize, xy: [(f64, f64); n] }

    let mut ans = 0;
    for i in 0..n {
        for j in i+1..n {
            let tmp = (xy[j].1 - xy[i].1) / (xy[j].0 - xy[i].0);
            if tmp < -1.0 || 1.0 < tmp {
                continue;
            } else {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}
