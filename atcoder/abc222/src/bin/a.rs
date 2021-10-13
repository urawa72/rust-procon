use proconio::{input, fastout};

#[fastout]
fn main() {
    input! { n: usize }
    println!("{:0pad$}", n, pad = 4);
}
