use proconio::{input, fastout};

#[fastout]
fn main() {
    input! { s: usize, t: usize }

    let mut ans = 0;
    for i in 0..=100 {
        for j in 0..=100 {
            for k in 0..=100 {
                if i + j + k <= s && i * j * k <= t {
                    ans += 1;
                }
            }
        }
    }
    println!("{}", ans);
}
