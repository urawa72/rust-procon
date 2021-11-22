#![allow(unused_imports)]
#![allow(dead_code)]
use itertools::Itertools;
use proconio::{fastout, input, marker::*};
use std::{cmp::*, collections::*, mem};
const MOD: usize = 1_000_000_007;

#[fastout]
fn main() {
    input! { s: Chars }
    if s[0] == s[1] && s[1] == s[2] {
        println!("Won");
    } else {
        println!("Lost");
    }
}
