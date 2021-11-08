#![allow(unused_imports)]
#![allow(dead_code)]
use itertools::Itertools;
use proconio::{fastout, input, marker::*};
use std::{cmp::*, collections::*, mem};
const MOD: usize = 1_000_000_007;

#[fastout]
fn main() {
    input! { n: usize, m: usize, v: [[usize; m]; n] }

    // 行チェック
    for i in 0..n {
        for j in 0..m - 1 {
            if v[i][j] + 1 == v[i][j + 1] {
                continue;
            } else {
                println!("No");
                return;
            }
        }
    }

    // 列チェック
    for j in 0..m {
        for i in 0..n - 1 {
            if v[i][j] + 7 == v[i + 1][j] {
                continue;
            } else {
                println!("No");
                return;
            }
        }
    }

    // 7の倍数があれば一番右端かチェック
    for j in 0..m {
        if v[0][j] % 7 == 0 && j != m - 1 {
            println!("No");
            return;
        }
    }

    println!("Yes");
}
