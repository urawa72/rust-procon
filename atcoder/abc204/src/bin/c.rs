#![allow(unused_imports)]
#![allow(dead_code)]
use itertools::Itertools;
use proconio::{fastout, input, marker::*};
use std::{cmp::*, collections::*, mem, ops::Add};
const MOD: usize = 1_000_000_007;

#[fastout]
fn main() {
    input! { n: usize, m: usize, ab: [(usize, usize); m] }
    let mut g = vec![vec![]; n];
    for i in 0..m {
        g[ab[i].0 - 1].push(ab[i].1 - 1);
    }

    let mut sum = 0;
    for i in 0..n {
        let mut seen = vec![false; n];
        dfs(i, &mut sum, &g, &mut seen);
    }
    println!("{}", sum);
}

fn dfs(v: usize, sum: &mut usize, g: &Vec<Vec<usize>>, seen: &mut Vec<bool>) {
    *sum += 1;
    seen[v] = true;
    for &next_v in &g[v] {
        if seen[next_v] {
            continue;
        }
        dfs(next_v, sum, g, seen);
    }
}
