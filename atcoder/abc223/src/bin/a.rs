use proconio::{fastout, input};

#[fastout]
fn main() {
    input! { x: usize }
    println!("{}", if x != 0 && x % 100 == 0 { "Yes" } else { "No" });
}
