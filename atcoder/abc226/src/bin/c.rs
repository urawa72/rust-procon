#![allow(unused_imports)]
#![allow(dead_code)]
use itertools::Itertools;
use proconio::{fastout, input, marker::*};
use std::{
    cmp::*,
    collections::*,
    mem::{self, replace},
};
const MOD: usize = 1_000_000_007;

#[fastout]
fn main() {
    input! { n: usize }

    let mut tv = vec![0usize; n];
    let mut kv = vec![0usize; n];
    let mut av = vec![vec![0usize]; n];

    for i in 0..n {
        input! { t: usize, k: usize }
        tv[i] = t;
        kv[i] = k;
        if 0 < k {
            input! { a: [usize; k] }
            av[i] = a;
        } else {
            av[i] = vec![];
        }
    }

    let mut used = vec![false; n];
    used[n - 1] = true;
    let mut ans = 0;
    for i in (0..n).rev() {
        if used[i] {
            ans += tv[i];
            if 0 < av[i].len() {
                for &j in av[i].iter() {
                    used[j - 1] = true;
                }
            }
        }
    }
    println!("{}", ans);
}
