#![allow(unused_imports)]
#![allow(dead_code)]
use itertools::Itertools;
use proconio::{fastout, input, marker::*};
use std::{cmp::*, collections::*, mem};
const MOD: usize = 1_000_000_007;

#[fastout]
fn main() {
    input! { n: usize, a: [usize; n], b: [usize; n], c: [usize; n] }

    let mut m1: HashMap<usize, usize> = HashMap::new();
    let mut m2: HashMap<usize, usize> = HashMap::new();
    let mut m3: HashMap<usize, usize> = HashMap::new();

    for x in a {
        let e = m1.entry(x % 46).or_insert(0);
        *e += 1;
    }
    for x in b {
        let e = m2.entry(x % 46).or_insert(0);
        *e += 1;
    }
    for x in c {
        let e = m3.entry(x % 46).or_insert(0);
        *e += 1;
    }

    let mut ans: usize = 0;
    for (k1, v1) in &m1 {
        for (k2, v2) in &m2 {
            for (k3, v3) in &m3 {
                if (k1 + k2 + k3) % 46 == 0 {
                    ans += v1 * v2 * v3;
                }
            }
        }
    }
    println!("{}", ans);
}
