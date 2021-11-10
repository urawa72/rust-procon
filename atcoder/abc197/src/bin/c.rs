#![allow(unused_imports)]
#![allow(dead_code)]
use itertools::Itertools;
use proconio::{fastout, input, marker::*};
use std::{cmp::*, collections::*, mem};
const MOD: usize = 1_000_000_007;

#[fastout]
fn main() {
    input! { n: usize, a: [usize; n] }

    let mut ans = usize::max_value();
    // 分けるところをbit全探索
    for bit in 0..1 << n {
        let mut sum = 0;
        let mut cur = a[0];
        for i in 1..n {
            if (bit & 1 << i) != 0 {
                sum ^= cur;
                cur = 0;
            }
            cur |= a[i];
        }
        ans = min(ans, sum ^ cur);
    }
    println!("{}", ans);

}
