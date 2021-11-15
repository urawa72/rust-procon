#![allow(unused_imports)]
#![allow(dead_code)]
use itertools::Itertools;
use proconio::{fastout, input, marker::*};
use std::{cmp::*, collections::*, mem};
const MOD: usize = 1_000_000_007;

#[fastout]
fn main() {
    input! { n: usize }

    let mut ans = 0;
    if 1_000 <= n {
        ans += n - 999;
    }
    if 1_000_000 <= n {
        ans += n - 999_999;
    }
    if 1_000_000_000 <= n {
        ans += n - 999_999_999;
    }
    if 1_000_000_000_000 <= n {
        ans += n - 999_999_999_999;
    }
    if 1_000_000_000_000_000 <= n {
        ans += n - 999_999_999_999_999;
    }

    println!("{}", ans);
}
