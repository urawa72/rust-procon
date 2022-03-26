#![allow(unused_imports)]
#![allow(dead_code)]
use im_rc::HashMap;
use itertools::Itertools;
use proconio::{fastout, input, marker::*};
use std::{cmp::*, collections::*, mem};
const MOD: usize = 1_000_000_007;

#[fastout]
fn main() {
    input! { n: usize, q: usize, a: [usize; n] }

    let mut map: HashMap<usize, Vec<usize>> = HashMap::new();
    for i in 0..n {
        let e = map.entry(a[i]).or_insert(vec![]);
        e.push(i + 1);
    }

    for _ in 0..q {
        input! { x: usize, k: usize }
        match map.get(&x) {
            Some(v) => {
                if k <= v.len() {
                    println!("{}", v[k - 1]);
                } else {
                    println!("-1");
                }
            }
            None => println!("-1"),
        }
    }
}
