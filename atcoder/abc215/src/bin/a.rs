use proconio::{input, fastout};

#[fastout]
fn main() {
    input! { s: String }
    println!("{}", if s == "Hello,World!" { "AC" } else { "WA" });
}
