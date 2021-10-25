use proconio::{fastout, input};

#[fastout]
fn main() {
    input! { s: String }
    let n = s.len();
    if s[n - 2..].to_string() == "er" {
        println!("er");
        return;
    } else {
        println!("ist");
    }
}
