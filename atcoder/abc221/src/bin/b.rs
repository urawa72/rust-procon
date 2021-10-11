use proconio::{input, fastout, marker::Chars};

#[fastout]
fn main() {
    input! { s: Chars, t: Chars }

    for i in 0..s.len() - 1 {
        let mut tmps = s.clone();
        tmps.swap(i, i + 1);
        if tmps == t || s == t {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
