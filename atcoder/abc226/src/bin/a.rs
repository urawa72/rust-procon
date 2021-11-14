#![allow(unused_imports)]
#![allow(dead_code)]
use itertools::Itertools;
use num::ToPrimitive;
use proconio::{fastout, input, marker::*};
use std::{cmp::*, collections::*, mem};
const MOD: usize = 1_000_000_007;

#[fastout]
fn main() {
    input!{ x: f64 }
    println!("{}", x.round());
}
