#![allow(unused_imports)]
#![allow(dead_code)]
use itertools::Itertools;
use proconio::{fastout, input, marker::*};
use std::{cmp::*, collections::*, mem};
const MOD: usize = 1_000_000_007;

#[fastout]
fn main() {
    input! { mut n: usize, a: [usize; n] }

    let mut map = HashMap::new();
    for i in a.iter() {
        *map.entry(i).or_insert(0) += 1;
    }

    let mut ans = 0;
    for i in a.iter() {
        let tmp = n - *map.entry(i).or_insert(0);
        ans += tmp;
        n -= 1;
        *map.entry(i).or_insert(0) -= 1;
    }
    println!("{}", ans);
}
