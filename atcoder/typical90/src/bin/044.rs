#![allow(unused_imports)]
#![allow(dead_code)]
use itertools::Itertools;
use proconio::{fastout, input, marker::*};
use std::{cmp::*, collections::*, mem};
const MOD: usize = 1_000_000_007;

#[fastout]
fn main() {
    input! { n: usize, q: usize, mut a: [usize; n] }
    let mut p = 0;
    for _ in 0..q {
        input! { t: usize, mut x: usize, mut y: usize }
        if t == 1 {
            x -= 1;
            y -= 1;
            x = (x + p) % n;
            y = (y + p) % n;
            let tmp = a[y];
            a[y] = a[x];
            a[x] = tmp;
        }
        if t == 2 {
            // 配列の開始位置の添字を-1ずつずらす
            p = (p + n - 1) % n;
        }
        if t == 3 {
            x -= 1;
            println!("{}", a[(x + p) % n]);
        }
    }
}
