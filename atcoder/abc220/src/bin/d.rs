use proconio::{input, fastout};

#[fastout]
fn main() {
    input! { n: usize, a: [usize; n] }
    const M: usize = 998244353;
    let mut dp = vec![vec![0_usize; 10]; n];

    dp[0][a[0]] = 1;
    for i in 0..n - 1 {
        for j in 0..10 {
            dp[i + 1][(j + a[i + 1]) % 10] += dp[i][j] % M;
            dp[i + 1][(j * a[i + 1]) % 10] += dp[i][j] % M;
        }
    }

    for i in 0..10 {
        println!("{}", dp[n - 1][i] % M);
    }
}
