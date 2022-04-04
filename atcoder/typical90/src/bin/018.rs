#![allow(unused_imports)]
#![allow(dead_code)]
use itertools::Itertools;
use libm::{hypot, atan2};
use proconio::{fastout, input, marker::*};
use std::{cmp::*, collections::*, mem};
const MOD: usize = 1_000_000_007;

#[fastout]
fn main() {
    input! { t: f64, l: f64, x: f64, y: f64, q: usize }

    for _ in 0..q {
        input! { e: f64 }
        let theta = (360.0 * e / t).to_radians();
        // E8が存在する座標(x, y, z)
        let pos = (0.0, -l / 2.0 * theta.sin(), -l / 2.0 * theta.cos() + l / 2.0);
        // posから(x,y)までの距離
        let dis = hypot(x, y - pos.1);
        // 2辺から角度を求める
        let ans = atan2(pos.2, dis).to_degrees();
        println!("{}", ans);
    }
}
