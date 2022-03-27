#![allow(unused_imports)]
#![allow(dead_code)]
use itertools::Itertools;
use proconio::{fastout, input, marker::*};
use std::{cmp::*, collections::*, mem};
const MOD: usize = 1_000_000_007;

#[fastout]
fn main() {
    input! { n: usize, l: isize, k: usize, a: [isize; n] }

    // 全ての長さをx以上にすることが可能か
    let check = |x: isize| {
        let mut cut_cnt = 0;
        let mut pre = 0;
        for i in 0..n {
            // x 以上なら切断
            if a[i] - pre >= x {
                cut_cnt += 1;
                pre = a[i];
            }
        }
        // 最後のピースが x 以上なら加算
        if l - pre >= x {
            cut_cnt += 1;
        }
        // ピースが k + 1 以上なら可能
        k + 1 <= cut_cnt
    };

    // 二分探索でスコアの最大を探す
    // lは常にそのスコア以上を実現可能 -> もっと大きいスコアで試す
    // rは常にそのスコアを実現不可能 -> もっと小さいスコアで試す
    let mut left: isize = -1;
    let mut right: isize = l + 1;
    while right - left > 1 {
        let mid = (left + right) / 2;
        if check(mid) {
            left = mid;
        } else {
            right = mid;
        }
    }
    println!("{}", left);
}
