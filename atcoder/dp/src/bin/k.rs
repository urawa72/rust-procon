use proconio::{fastout, input};

#[fastout]
fn main() {
    input! { n: usize, k: usize, a: [usize; n] }
    let mut dp = vec![0usize; k + 1];

    for i in 0..=k {
        for j in 0..n {
            if a[j] <= i && dp[i - a[j]] == 0 {
                dp[i] = 1;
            }
        }
    }

    println!("{}", if dp[k] == 1 { "First" } else { "Second" });
}
