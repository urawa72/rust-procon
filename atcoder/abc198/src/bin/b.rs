#![allow(unused_imports)]
#![allow(dead_code)]
use itertools::Itertools;
use proconio::{fastout, input, marker::*};
use std::{cmp::*, collections::*, mem};
const MOD: usize = 1_000_000_007;

pub trait ExtendString {
    fn reverse(&self) -> String;
}
impl ExtendString for String {
    fn reverse(&self) -> String {
        self.chars().rev().collect::<String>()
    }
}

#[fastout]
fn main() {
    input! { n: String }

    let m = n.reverse().parse::<usize>().unwrap().to_string().reverse();
    let l = n.reverse().parse::<usize>().unwrap().to_string();
    if m == l {
        println!("Yes");
    } else {
        println!("No");
    }
}
