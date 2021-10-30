#![allow(unused_imports)]
#![allow(dead_code)]
use itertools::Itertools;
use proconio::{fastout, input, marker::*};
use std::{cmp::*, collections::*, mem};
const MOD: usize = 1_000_000_007;

#[fastout]
fn main() {
    input! { x: usize, y: usize }
    if x == y {
        println!("{}", x);
        return;
    }
    for i in 0..3 {
        if x == i || y == i {
            continue;
        } else {
            println!("{}", i);
            return;
        }
    }
}
