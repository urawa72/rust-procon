#![allow(unused_imports)]
#![allow(dead_code)]
use itertools::Itertools;
use proconio::{fastout, input, marker::*};
use std::{cmp::*, collections::*, mem};
const MOD: usize = 1_000_000_007;

#[fastout]
fn main() {
    input! { n: usize, mut a: [usize; n] }

    let mut map = HashMap::new();
    for i in 0..n {
        a[i] %= 200;
        *map.entry(a[i]).or_insert(0) += 1;
    }

    let mut ans: usize = 0;
    for i in 0..n {
        let e = map.entry(a[i]).or_insert(0);
        *e -= 1;
        ans += *e;
    }
    println!("{}", ans);
}
