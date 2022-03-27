#![allow(unused_imports)]
#![allow(dead_code)]
use itertools::Itertools;
use proconio::{fastout, input, marker::*};
use std::{cmp::*, collections::*, mem};
const MOD: usize = 1_000_000_007;

#[fastout]
fn main() {
    input! { n: usize, m: usize, s: [String; n], t: [String; m] }

    let mut map = HashMap::new();
    for st in t {
        map.entry(st).or_insert(true);
    }

    for st in s {
        match map.get(&st) {
            Some(_) => {
                println!("Yes")
            }
            None => {
                println!("No")
            }
        }
    }
}
