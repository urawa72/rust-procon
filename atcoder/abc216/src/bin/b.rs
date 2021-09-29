use std::collections::HashSet;

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! { n: usize, st: [(String, String); n] }

    let d = st.iter().cloned().collect::<HashSet<(_, _)>>();
    println!("{}", if d.len() == n { "No" } else { "Yes" });
}
