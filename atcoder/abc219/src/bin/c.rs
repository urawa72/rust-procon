use im_rc::HashMap;
use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! { x: String, n: i64, s: [String; n] }

    // index付きでループする
    let x = x.chars().zip(0_i32..).collect::<HashMap<_, _>>();

    let s2 = s
        .iter()
        .map(|v| v.chars().map(|c| x[&c]).collect_vec())
        .zip(0..)
        .sorted();

    for t in s2 {
        println!("{}", s[t.1]);
    }
}
