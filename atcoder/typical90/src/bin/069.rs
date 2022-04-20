#![allow(unused_imports)]
#![allow(dead_code)]
use itertools::Itertools;
use proconio::{fastout, input, marker::*};
use std::{cmp::*, collections::*, mem};
const MOD: usize = 1_000_000_007;

#[fastout]
fn main() {
    input! { n: usize, k: usize }

    let mut ans = 1;
    if 3 <= k {
        if 3 <= n {
            ans *= mod_pow(k - 2, n - 2, MOD);
            ans *= k;
            ans %= MOD;
            ans *= k - 1;
            ans %= MOD;
        } else {
            if n == 2 {
                ans = (ans * k) % MOD;
                ans = (ans * (k - 1)) % MOD;
            }
            if n == 1 {
                ans = (ans * k) % MOD;
            }
        }
    } else {
        if 3 <= n {
            ans = 0;
        }
        if n == 2 {
            ans = (ans * k) % MOD;
            ans = (ans * (k - 1)) % MOD;
        }
        if n == 1 {
            ans = (ans * k) % MOD;
        }
    }
    println!("{}", ans);
}

// 繰り返し二乗法
fn mod_pow(a: usize, n: usize, p: usize) -> usize {
    if n == 0 {
        return 1;
    }
    if n == 1 {
        return  a % p;
    }
    if n % 2 == 1 {
        return (a * mod_pow(a, n - 1, p)) % p;
    }
    let t = mod_pow(a , n / 2, p);
    return (t * t) % p;
}
