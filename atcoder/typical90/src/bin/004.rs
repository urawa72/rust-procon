#![allow(unused_imports)]
#![allow(dead_code)]
use itertools::Itertools;
use proconio::{fastout, input, marker::*};
use std::{cmp::*, collections::*, mem};
const MOD: usize = 1_000_000_007;

#[fastout]
fn main() {
    input! { h: usize, w: usize, b: [[usize; w]; h] }

    let mut y = vec![];
    for i in 0..h {
        let s: usize = b[i].iter().sum();
        y.push(s);
    }
    let mut x = vec![];
    for i in 0..w {
        let mut tmp = 0;
        for j in 0..h {
            tmp += b[j][i];
        }
        x.push(tmp);
    }

    for i in 0..h {
        for j in 0..w {
            print!("{}", y[i] + x[j] - b[i][j]);
            if j != w - 1 {
                print!(" ");
            }
        }
        println!()
    }
}
