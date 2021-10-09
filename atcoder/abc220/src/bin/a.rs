use proconio::{input, fastout};

#[fastout]
fn main() {
    input! { a: usize, b: usize, c: usize }
    for i in a..=b {
        if i % c == 0 {
            println!("{}", i);
            return;
        }
    }
    println!("{}", -1);
}
