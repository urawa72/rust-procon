// use std::collections::HashMap;

use itertools::Itertools;
use proconio::{input, fastout};

#[fastout]
fn main() {
    input! { p: [usize; 26] };
    // let s = "abcdefghijklmnopqrstuvwxyz".to_string();
    // let m = (1..).zip(s.chars()).collect::<HashMap<_, _>>();
    // let ans = p.iter().map(|n| m[n]).collect::<String>();
    let v = (b'a'..=b'z').map(|c| c as char).collect_vec();
    println!("{}", p.iter().map(|n| &v[*n - 1]).collect::<String>());
}
