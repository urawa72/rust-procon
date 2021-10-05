use proconio::{input, fastout};

#[fastout]
fn main() {
    input! { n: usize, a: usize, x: usize, y: usize }
    if a <= n {
        println!("{}", a * x + (n - a) * y);
    } else {
        println!("{}", n * x);
    }
}
