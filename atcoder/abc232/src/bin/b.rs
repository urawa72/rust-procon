#![allow(unused_imports)]
#![allow(dead_code)]
use itertools::Itertools;
use proconio::{fastout, input, marker::*};
use std::{cmp::*, collections::*, mem};
const MOD: usize = 1_000_000_007;

#[fastout]
fn main() {
    input! { s: String, t: String }
    let s = s.chars().map(|e| e as i32).collect::<Vec<_>>();
    let t = t.chars().map(|e| e as i32).collect::<Vec<_>>();
    let mut pre = 100;
    for i in 0..s.len() {
        let mut d = t[i] - s[i];
        if d < 0 {
            d += 26;
        }
        if pre != 100 && d != pre {
            println!("No");
            return;
        }
        pre = d;
    }
    println!("Yes");
}
