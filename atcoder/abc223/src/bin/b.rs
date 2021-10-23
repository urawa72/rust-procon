use proconio::{input, fastout};

#[fastout]
fn main() {
    input! { s: String }

    let mut ans = Vec::new();
    for i in 0..s.len() {
        println!("{:?}, {:?}", s[i..].to_string(), &s[0..i]);
        ans.push(s[i..].to_string() + &s[0..i]);
    }
    ans.sort();
    println!("{}\n{}", ans[0], ans[s.len() - 1]);
}
