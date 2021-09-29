use proconio::{fastout, input};

pub trait ExtendString {
    fn reverse(&self) -> String;
}

impl ExtendString for String {
    fn reverse(&self) -> String {
        self.chars().rev().collect::<String>()
    }
}

#[fastout]
fn main() {
    input! { n: usize }
    let mut ans = String::new();
    let mut n = n;
    while n > 0 {
        if n % 2 != 0 {
            ans.push_str("A");
            n -= 1;
        } else {
            ans.push_str("B");
            n /= 2;
        }
    }
    let ans = ans.reverse();
    println!("{}", ans);
}
