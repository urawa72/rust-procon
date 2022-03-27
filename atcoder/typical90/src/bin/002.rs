#![allow(unused_imports)]
#![allow(dead_code)]
use itertools::Itertools;
use proconio::{fastout, input, marker::*};
use std::{cmp::*, collections::*, mem};
const MOD: usize = 1_000_000_007;

#[fastout]
fn main() {
    input! { n: usize }

    for bit in 0..(1 << n) {
        let mut v = vec![];
        for i in (0..n).rev() {
            if bit & 1 << i != 0 {
                v.push(")");
            } else {
                v.push("(");
            }
        }

        // [典型] 正しいカッコ列の条件
        // '('と')'の数が同じ
        // 左からi文字目までの時点で'('の数が')'の数以上である
        // '('の数 - ')'の数 が0以上なら上記を満たす
        let mut sum = 0;
        for &s in &v {
            if s == "(" {
                sum += 1;
            } else {
                sum -= 1;
            }
            if sum < 0 {
                break;
            }
        }
        if sum == 0 {
            println!("{}", v.iter().join(""));
        }
    }
}
