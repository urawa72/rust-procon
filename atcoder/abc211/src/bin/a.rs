use proconio::{input, fastout};

#[fastout]
fn main() {
    input! { a: f64, b: f64 }
    println!("{}", (a - b) / 3.0 + b);
}
