#![allow(unused_imports)]
#![allow(dead_code)]
use itertools::Itertools;
use proconio::{fastout, input, marker::*};
use std::{cmp::*, collections::*, mem};
const MOD: usize = 1_000_000_007;

#[fastout]
fn main() {
    input! { n: usize, mut s: String, q: usize }
    let t = s.split_off(n);
    let mut s = s.chars().collect_vec();
    let mut t = t.chars().collect_vec();
    let mut f = false;
    for _ in 0..q {
        input! { x: usize, mut a: usize, mut b: usize }
        if x == 1 {
            if a <= n && b <= n {
                if f {
                    change(&mut t, a - 1, b - 1);
                } else {
                    change(&mut s, a - 1, b - 1);
                }
            } else if n < a && n < b {
                if f {
                    change(&mut s, a - n - 1, b - n - 1);
                } else {
                    change(&mut t, a - n - 1, b - n - 1);
                }
            } else {
                if n < a {
                    a -= n;
                }
                if n < b {
                    b -= n;
                }
                if f {
                    change_both(&mut t, &mut s, a - 1, b - 1);
                } else {
                    change_both(&mut s, &mut t, a - 1, b - 1);
                }
            }
        } else {
            f = !f;
        }
    }

    if f {
        println!("{}{}", t.iter().join(""), s.iter().join(""));
    } else {
        println!("{}{}", s.iter().join(""), t.iter().join(""));
    }
}

fn change(s: &mut Vec<char>, a: usize, b: usize) {
    let c1 = s[a];
    let c2 = s[b];
    s[a] = c2;
    s[b] = c1;
}

fn change_both(s: &mut Vec<char>, t: &mut Vec<char>, a: usize, b: usize) {
    let c1 = s[a];
    let c2 = t[b];
    s[a] = c2;
    t[b] = c1;
}
