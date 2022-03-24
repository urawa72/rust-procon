#![allow(unused_imports)]
#![allow(dead_code)]
use itertools::Itertools;
use proconio::{fastout, input, marker::*};
use std::{cmp::*, collections::*, mem};
const MOD: usize = 1_000_000_007;

#[fastout]
fn main() {
    input! { n: usize, x: usize }
    let mut v = vec![vec![]; n];
    for i in 0..n {
        input! { l: usize }
        for _ in 0..l {
            input! { a: usize }
            v[i].push(a);
        }
    }

    let ans = dfs(1, 0, n, x, &v);
    println!("{}", ans);
}

fn dfs(s: usize, i: usize, n: usize, x: usize, v: &Vec<Vec<usize>>) -> usize {
    if i == n {
        if s == x {
            return 1;
        }
        return 0;
    }
    let mut cnt = 0;
    for j in 0..v[i].len() {
        if x / v[i][j] < s {
            continue;
        }
        cnt += dfs(s * v[i][j], i + 1, n, x, v);
    }
    cnt
}
