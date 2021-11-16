#![allow(unused_imports)]
#![allow(dead_code)]
use itertools::Itertools;
use num_traits::Pow;
use proconio::{fastout, input, marker::*};
use std::{cmp::*, collections::*, mem};
const MOD: usize = 1_000_000_007;

#[fastout]
fn main() {
    input! { n: usize, a: [isize; n] }

    let mut map = HashMap::new();
    for &i in a.iter() {
        *map.entry(i).or_insert(0) += 1;
    }

    let mut ans = 0;
    for i in 0..n {
        let e = map.entry(a[i]).or_insert(0);
        *e -= 1;
        for (&k, &v) in map.iter() {
            if 0 < v {
                ans += (k - a[i]) * (k - a[i]) * v;
            }
        }
    }
    println!("{}", ans);
}
