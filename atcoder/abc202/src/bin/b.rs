#![allow(unused_imports)]
#![allow(dead_code)]
use itertools::Itertools;
use proconio::{fastout, input, marker::*};
use std::{cmp::*, collections::*, mem};
const MOD: usize = 1_000_000_007;

#[fastout]
fn main() {
    input! { s: Chars }

    for &c in s.iter().rev() {
        if c == '9' {
            print!("6");
        } else if c == '6' {
            print!("9");
        } else {
            print!("{}", c);
        }
    }
    print!("\n");
}
