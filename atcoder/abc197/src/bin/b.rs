#![allow(unused_imports)]
#![allow(dead_code)]
use itertools::Itertools;
use proconio::{fastout, input, marker::*};
use std::{cmp::*, collections::*, mem};
const MOD: usize = 1_000_000_007;

#[fastout]
fn main() {
    input! { h: usize, w: usize, mut x: usize, mut y: usize, g: [Chars; h] }
    x -= 1;
    y -= 1;

    let mut ans = 1;
    let mut c = 0;
    for i in 0..w {
        if g[x][i] == '#' {
            if y < i {
                ans += c;
                c = 0;
                break;
            }
            c = 0;
        } else if i == y {
            ans += c;
            c = 0;
        } else {
            c += 1;
        }
    }
    if 0 < c {
        ans += c;
    }
    c = 0;
    for j in 0..h {
        if g[j][y] == '#' {
            if x < j {
                ans += c;
                c = 0;
                break;
            }
            c = 0;
        } else if j == x {
            ans += c;
            c = 0;
        } else {
            c += 1;
        }
    }
    if 0 < c {
        ans += c;
    }
    println!("{}", ans);
}
