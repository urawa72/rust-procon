use num::pow;
use proconio::{input, fastout};

#[fastout]
fn main() {
    input! { a: usize, b: usize }
    println!("{}", pow(32, a - b));
}
