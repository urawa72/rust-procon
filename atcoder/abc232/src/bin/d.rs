#![allow(unused_imports)]
#![allow(dead_code)]
use itertools::Itertools;
use proconio::{fastout, input, marker::*};
use std::{cmp::*, collections::*, mem};
const MOD: usize = 1_000_000_007;

#[fastout]
fn main() {
    input! { h: usize, w: usize, b: [String; h] }

    let mut board = vec![];
    for s in b {
        board.push(s.chars().collect::<Vec<_>>());
    }

    let mut dist = vec![vec![0; w]; h];
    let mut q: VecDeque<(usize, usize)> = VecDeque::new();
    let mut ans = 0;

    dist[0][0] = 1;
    q.push_back((0, 0));

    while !q.is_empty() {
        match q.pop_front() {
            Some((y, x)) => {
                ans = max(ans, dist[y][x]);
                for &(dy, dx) in &[(0, 1), (1, 0)] {
                    let ny = y + dy;
                    let nx = x + dx;
                    if h <= ny || w <= nx {
                        continue;
                    }
                    if board[ny][nx] == '#' {
                        continue;
                    }
                    if dist[ny][nx] == 0 {
                        dist[ny][nx] = dist[y][x] + 1;
                        q.push_back((ny, nx));
                    }
                }
            }
            None => break,
        }
    }

    println!("{}", ans);
}
