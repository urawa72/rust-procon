#![allow(unused_imports)]
#![allow(dead_code)]
use itertools::Itertools;
use proconio::{fastout, input, marker::*};
use std::{cmp::*, collections::*, mem};
const MOD: usize = 1_000_000_007;

#[fastout]
fn main() {
    input! { n: usize, s: [Chars; n] }

    let mut map = HashMap::new();
    for i in 0..n {
        *map.entry(s[i].iter().collect::<String>()).or_insert(0) += 1;
    }

    for i in 0..n {
        if s[i][0] != '!' {
            let t = "!".to_string() + s[i].iter().collect::<String>().as_str();
            if *map.entry(t).or_insert(0) != 0 {
                println!("{}", s[i].iter().collect::<String>());
                return;
            }
        }
    }
    println!("satisfiable");
}
