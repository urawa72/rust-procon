use proconio::{input, fastout};

// 期待値DP
#[fastout]
fn main() {
    input! { n: usize, a: [usize; n] }

    let mut count = vec![0usize; 4];
    for i in 0..n {
        count[a[i]] += 1;
    }


    // 寿司が残り1個の皿がc1枚、2個の皿がc2枚、3個の皿がc3枚の状態から、
    // 寿司をすべてなくすのに必要な操作回数の期待値
    let mut dp = vec![vec![vec![0f64; 303]; 303]; 303];
    for c3 in 0..=n {
        for c2 in 0..=n {
            for c1 in 0..=n {
                let sum = c1 + c2 + c3;
                if sum == 0 {
                    continue;
                }

                dp[c1][c2][c3] = 1.0 * n as f64 / sum as f64;
                if 0 < c1 {
                    dp[c1][c2][c3] += dp[c1 - 1][c2][c3] * c1 as f64 / sum as f64;
                }
                if 0 < c2 {
                    // 2個皿を-1するので1個皿が+1になる
                    dp[c1][c2][c3] += dp[c1 + 1][c2 - 1][c3] * c2 as f64 / sum as f64;
                }
                if 0 < c3 {
                    // 3個皿を-1するので2個皿が+1になる
                    dp[c1][c2][c3] += dp[c1][c2 + 1][c3 - 1] * c3 as f64 / sum as f64;
                }
            }
        }
    }

    println!("{}", dp[count[1]][count[2]][count[3]]);
}
