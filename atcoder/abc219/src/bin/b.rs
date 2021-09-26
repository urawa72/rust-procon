use proconio::{fastout, input};

#[fastout]
fn main() {
    input! { s1: String, s2: String, s3: String, t: String }

    let mut ans = "".to_string();
    for c in t.chars() {
        if c == '1' {
            ans.push_str(&s1);
        }
        if c == '2' {
            ans.push_str(&s2);
        }
        if c == '3' {
            ans.push_str(&s3);
        }
    }

    println!("{}", ans);
}
