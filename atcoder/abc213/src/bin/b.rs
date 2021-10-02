use std::collections::HashMap;

use itertools::Itertools;
use proconio::{input, fastout};

#[fastout]
fn main() {
    input! { n: usize, a: [usize; n] }
    let m = a.iter().zip(0..).collect::<HashMap<_, _>>();
    let v = a.iter().sorted().collect_vec();
    println!("{}", m.get(&v[n - 2]).unwrap() + 1);
}
