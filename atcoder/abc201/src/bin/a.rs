#![allow(unused_imports)]
#![allow(dead_code)]
use itertools::Itertools;
use proconio::{fastout, input, marker::*};
use std::{cmp::*, collections::*, mem};
const MOD: usize = 1_000_000_007;

#[fastout]
fn main() {
    input! { mut a: [usize; 3] }
    a.sort();
    if a[1] - a[0] == a[2] - a[1] {
        println!("Yes");
    } else {
        println!("No");
    }
}
