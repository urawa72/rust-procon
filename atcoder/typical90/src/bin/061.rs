#![allow(unused_imports)]
#![allow(dead_code)]
use itertools::Itertools;
use proconio::{fastout, input, marker::*};
use std::{cmp::*, collections::*, mem};
const MOD: usize = 1_000_000_007;

#[fastout]
fn main() {
    input! { q: usize, tx: [(usize, usize); q] }
    let mut deq = VecDeque::new();

    for (t, x) in tx {
        if t == 1 {
            deq.push_front(x);
        }
        if t == 2 {
            deq.push_back(x);
        }
        if t == 3{
            println!("{}", deq.get(x - 1).unwrap());
        }
    }
}
