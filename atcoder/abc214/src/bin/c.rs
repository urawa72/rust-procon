use itertools::Itertools;
use proconio::{fastout, input};
use std::cmp::min;

#[fastout]
fn main() {
    input! { n: usize, ss: [usize; n], mut tt: [usize; n] }

    for i in 0..2 * n {
        tt[(i + 1) % n] = min(tt[(i + 1) % n], tt[i % n] + ss[i % n]);
    }
    println!("{}", tt.iter().join("\n"));
}
