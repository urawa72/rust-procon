#![allow(unused_imports)]
#![allow(dead_code)]
use itertools::Itertools;
use proconio::{fastout, input, marker::*};
use std::{cmp::*, collections::*, mem};
const MOD: usize = 1_000_000_007;

#[fastout]
fn main() {
    input! { n: usize, q: usize }

    let mut a = vec![0isize; n + 1];
    for i in 1..=n {
        input! { x: isize }
        a[i] = x;
    }

    let mut sum = 0;
    let mut b = vec![0isize; n + 1];
    for i in 1..n {
        b[i] = a[i + 1] - a[i];
        sum += b[i].abs();
    }

    for _ in 0..q {
        input! { mut l: isize, mut r: isize, v: isize }

        let mae = (b[(l - 1) as usize]).abs() + (b[r as usize]).abs();

        if 2 <= l {
            b[(l - 1) as usize] += v;
        }

        if r < n as isize {
            b[r as usize] -= v;
        }

        let ato = (b[(l - 1) as usize]).abs() + (b[r as usize]).abs();

        sum += ato - mae;

        println!("{}", sum);
    }
}
