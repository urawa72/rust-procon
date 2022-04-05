#![allow(unused_imports)]
#![allow(dead_code)]
use itertools::Itertools;
use permutohedron::LexicalPermutation;
use proconio::{fastout, input, marker::*};
use std::{cmp::*, collections::*, mem};
const MOD: usize = 1_000_000_007;

#[fastout]
fn main() {
    input! { n: usize, a: [[usize; n]; n], m: usize, x: [(usize, usize); m] }

    let mut g = vec![vec![false; n]; n];
    for (a, b) in x {
        g[a - 1][b - 1] = true;
        g[b - 1][a - 1] = true;
    }

    let mut ans = 1 << 30;
    let mut nums: Vec<usize> = (0..n).into_iter().collect();
    while {
        let mut sum = 0;
        for i in 0..n {
            sum += a[nums[i]][i];
        }
        let mut ok = true;
        for i in 0..n - 1 {
            if g[nums[i]][nums[i + 1]] {
                ok = false;
                break;
            }
        }
        if ok {
            ans = min(ans, sum);
        }
        nums.next_permutation()
    } {}

    if ans == 1 << 30 {
        println!("-1");
    } else {
        println!("{}", ans);
    }
}
