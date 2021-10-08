use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        a: usize,
        b: usize
    }
    println!("{}", if a <= b && b <= a * 6 { "Yes" } else { "No" });

}
