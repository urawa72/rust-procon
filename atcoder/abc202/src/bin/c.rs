#![allow(unused_imports)]
#![allow(dead_code)]
use itertools::Itertools;
use proconio::{fastout, input, marker::*};
use std::{cmp::*, collections::*, mem};
const MOD: usize = 1_000_000_007;

#[fastout]
fn main() {
    input! { n: usize, a: [usize;n], b: [usize; n], c: [usize; n] }

    let mut map1 = HashMap::new();
    for i in 0..n {
        *map1.entry(a[i]).or_insert(0) += 1;
    }
    let mut map2 = HashMap::new();
    for i in 0..n {
        *map2.entry(c[i] - 1).or_insert(0) += 1;
    }

    let mut ans = 0 as usize;
    for i in 0..n {
        let v = *map1.entry(b[i]).or_insert(0);
        if v == 0 {
            continue;
        }
        let w = *map2.entry(i).or_insert(0);
        if w == 0 {
            continue;
        }
        ans += v * w;
    }
    println!("{}", ans);
}
