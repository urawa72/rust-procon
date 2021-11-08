#![allow(unused_imports)]
#![allow(dead_code)]
use itertools::Itertools;
use proconio::{fastout, input, marker::*};
use std::{cmp::*, collections::*, mem};
const MOD: usize = 1_000_000_007;

#[fastout]
fn main() {
    input!{ s: String }
    let set = s.chars().collect::<HashSet<char>>();
    match set.len() {
        1 => println!("1"),
        2 => println!("3"),
        _ => println!("6")
    }
}
