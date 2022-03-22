#![allow(unused_imports)]
#![allow(dead_code)]
use itertools::Itertools;
use proconio::{fastout, input, marker::*};
use std::{cmp::*, collections::*, mem};
const MOD: usize = 1_000_000_007;

#[fastout]
fn main() {
    input! { n: usize, s: [String; n] }
    let mut m = HashMap::new();
    for t in s {
        let e = m.entry(t).or_insert(0);
        *e += 1;
    }
    println!("{}", m.iter().max_by(|b, a| b.1.cmp(a.1)).unwrap().0);
}
