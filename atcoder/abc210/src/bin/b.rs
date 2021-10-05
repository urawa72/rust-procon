use proconio::{input, fastout, marker::Chars};

#[fastout]
fn main() {
    input! { n: usize, s: Chars }
    for i in 0..n {
        if i % 2 == 0 && s[i] == '1' {
            println!("{}", "Takahashi");
            return;
        }
        if i % 2 != 0 && s[i] == '1' {
            println!("{}", "Aoki");
            return;
        }
    }
}
