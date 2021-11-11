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
    for i in 1..=1000000 {
        let mut j = i;
        let mut e = 0;
        while 0 < j {
            j /= 10;
            e += 1;
        }
        let m = i + i * 10_u32.pow(e as u32) as usize;
        if m <= n {
            ans += 1;
        }
    }
    println!("{}", ans);
}
