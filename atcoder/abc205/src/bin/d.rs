#![allow(unused_imports)]
#![allow(dead_code)]
use itertools::Itertools;
use num_derive::FromPrimitive;
use proconio::{fastout, input, marker::*};
use std::{cmp::*, collections::*, mem};
const MOD: usize = 1_000_000_007;

pub trait BinarySearch<T> {
    fn lower_bound(&self, x: &T) -> usize;
    fn upper_bound(&self, x: &T) -> usize;
}
impl<T: Ord> BinarySearch<T> for [T] {
    fn lower_bound(&self, x: &T) -> usize {
        let mut low = 0;
        let mut high = self.len();
        while low != high {
            let mid = (low + high) / 2;
            match self[mid].cmp(x) {
                Ordering::Less => low = mid + 1,
                Ordering::Equal | Ordering::Greater => high = mid,
            }
        }
        low
    }
    fn upper_bound(&self, x: &T) -> usize {
        let mut low = 0;
        let mut high = self.len();
        while low != high {
            let mid = (low + high) / 2;
            match self[mid].cmp(x) {
                Ordering::Less | Ordering::Equal => low = mid + 1,
                Ordering::Greater => high = mid,
            }
        }
        low
    }
}

#[fastout]
fn main() {
    input! { n: usize, q: usize, mut a: [usize; n], k: [usize; q] }

    let mut c = vec![0usize; n];
    for i in 0..n {
        c[i] = a[i] - i - 1;
    }

    for i in 0..q {
        let idx = c.lower_bound(&k[i]);
        if idx == 0 {
            println!("{}", k[i]);
        } else {
            println!("{}", a[idx - 1] + (k[i] - c[idx - 1]));
        }
    }
}
