use proconio::{fastout, input};

#[fastout]
fn main() {
    input! { a: usize, b: usize, c: usize }
    let mut ans = 0;
    if a != b && b != c && a != c {
        ans = 0;
    } else if a == b {
        ans = c;
    } else if b == c {
        ans = a;
    } else if a == c {
        ans = b;
    }
    println!("{}", ans);
}
