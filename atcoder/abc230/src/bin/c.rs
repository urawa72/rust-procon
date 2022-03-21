#![allow(unused_imports)]
#![allow(dead_code)]
use itertools::Itertools;
use proconio::{fastout, input, marker::*};
use std::{cmp::*, collections::*, mem};
const MOD: usize = 1_000_000_007;

#[fastout]
fn main() {
    input! { n: isize, a: isize, b: isize, p: isize, q: isize, r: isize, s: isize }

    let k1 = max(1 - a, 1 - b);
    let k2 = min(n - a, n - b);
    let k3 = max(1 - a, b - n);
    let k4 = min(n - a, b - 1);

    for i in p..=q {
        for j in r..=s {
            let mut c = ".";
            if a + k1 <= i && i <= a + k2 {
                if b + (i - a) == j {
                    c = "#";
                }
            }
            if a + k3 <= i && i <= a + k4 {
                if b - (i - a) == j {
                    c = "#";
                }
            }
            print!("{}", c);
        }
        println!();
    }
}
