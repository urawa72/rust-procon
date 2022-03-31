#![allow(unused_imports)]
#![allow(dead_code)]
use itertools::Itertools;
use num_traits::Pow;
use proconio::{fastout, input, marker::*};
use std::{cmp::*, collections::*, mem};
const MOD: usize = 1_000_000_007;

#[fastout]
fn main() {
    input! { mut n: String, k: usize }

    if n == "0".to_string() {
        println!("0");
        return;
    }

    for _ in 0..k {
        // 8 -> 10進数変換
        let mut sum = 0;
        let mut e = 1;
        for c in n.reverse().chars() {
            let num = c as u64 - 48;
            sum += num * e;
            e *= 8;
        }

        // 10 -> 9進数変換
        let mut v = vec![];
        while 0 < sum {
            v.push(sum % 9);
            sum /= 9;
        }
        v.reverse();
        let s: String = v
            .into_iter()
            .map(|i| if i == 8 { 5.to_string() } else { i.to_string() })
            .collect();
        n = s;
    }

    println!("{}", n);
}

pub trait ExtendString {
    fn reverse(&self) -> String;
}
impl ExtendString for String {
    fn reverse(&self) -> String {
        self.chars().rev().collect::<String>()
    }
}
