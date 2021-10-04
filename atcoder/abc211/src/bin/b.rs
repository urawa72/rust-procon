use std::collections::HashSet;

use proconio::{fastout, input};

#[fastout]
fn main() {
    let mut hs = HashSet::new();
    for _ in 0..4 {
        input! { s: String }
        hs.insert(s);
    }
    println!("{}", if hs.len() == 4 { "Yes" } else { "No" });
}
