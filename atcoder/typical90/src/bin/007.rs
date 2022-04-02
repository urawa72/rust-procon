#![allow(unused_imports)]
#![allow(dead_code)]
use itertools::Itertools;
use proconio::{fastout, input, marker::*};
use std::{cmp::*, collections::*, mem};
const MOD: usize = 1_000_000_007;

#[fastout]
fn main() {
    input! { n: usize, mut a: [isize; n], q: usize, b: [isize; q] }

    a.sort();

    for x in b.iter() {
        let i = a.lower_bound(x);
        if i == n {
            println!("{}", (x - a[i - 1]).abs());
        } else if 0 < i {
            println!("{}", min((x - a[i]).abs(), (x - a[i - 1]).abs()));
        } else {
            println!("{}", (x - a[i]).abs());
        }
    }
}

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
