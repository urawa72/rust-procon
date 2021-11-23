#![allow(unused_imports)]
#![allow(dead_code)]
use itertools::Itertools;
use num_traits::Pow;
use proconio::{fastout, input, marker::*};
use std::{cmp::*, collections::*, mem};
const MOD: usize = 1_000_000_007;

fn main() {
    input! { n: u32, a: [usize; 2_u32.pow(n)] }

    let mut a: Vec<(&usize, u32)> = a.iter().zip(0..2_u32.pow(n)).collect();
    while a.len() != 2 {
        let mut b = vec![];
        for j in 0..a.len() {
            if j % 2 != 0 {
                continue;
            }
            if a[j].0 < a[j + 1].0 {
                b.push(a[j + 1]);
            } else {
                b.push(a[j]);
            }
        }
        a = b;
    }

    if a[0].0 < a[1].0 {
        println!("{}", a[0].1 + 1);
    } else {
        println!("{}", a[1].1 + 1);
    }
}
