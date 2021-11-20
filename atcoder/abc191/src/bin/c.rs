#![allow(unused_imports)]
#![allow(dead_code)]
use itertools::Itertools;
use proconio::{fastout, input, marker::*};
use std::{cmp::*, collections::*, mem};
const MOD: usize = 1_000_000_007;

#[fastout]
fn main() {
    input! { h: usize, w: usize, s: [Chars; h] }

    let mut ans = 0;
    for i in 0..h - 1 {
        for j in 0..w - 1 {
            let mut cnt = 0;
            if s[i][j] == '#' {
                cnt += 1;
            }
            if s[i + 1][j] == '#' {
                cnt += 1;
            }
            if s[i][j + 1] == '#' {
                cnt += 1;
            }
            if s[i + 1][j + 1] == '#' {
                cnt += 1;
            }
            if cnt == 1 || cnt == 3 {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}
