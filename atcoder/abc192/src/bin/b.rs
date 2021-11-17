#![allow(unused_imports)]
#![allow(dead_code)]
use itertools::Itertools;
use proconio::{fastout, input, marker::*};
use std::{cmp::*, collections::*, mem};
const MOD: usize = 1_000_000_007;

#[fastout]
fn main() {
    input! { s: Chars }
    for i in 0..s.len() {
        if i % 2 != 0 {
            if 'A' <= s[i] && s[i] <= 'Z' {
                continue;
            } else {
                println!("No");
                return;
            }
        } else {
            if 'a' <= s[i] && s[i] <= 'z' {
                continue;
            } else {
                println!("No");
                return;
            }
        }
    }
    println!("Yes");
}
