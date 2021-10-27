use proconio::{fastout, input};
const MOD: usize = 1_000_000_007;

// 木DP
#[fastout]
fn main() {
    input! { n: usize }

    let mut dp = vec![vec![0usize; 2]; n + 10];
    let mut e = vec![vec![]; n + 10];
    for _ in 0..n - 1 {
        input! { mut x: usize, mut y: usize }
        x -= 1;
        y -= 1;
        e[x].push(y);
        e[y].push(x);
    }

    dfs(0, 1_000_000_000, &e, &mut dp);

    println!("{}", (dp[0][0] + dp[0][1]) % MOD);
}

// dp[i][j] = 頂点iを(j?Black:White)に塗ったときiを親とする部分木の塗り方の場合の数
fn dfs(i: usize, p: usize, e: &Vec<Vec<usize>>, dp: &mut Vec<Vec<usize>>) {
    dp[i][0] = 1;
    dp[i][1] = 1;
    for &j in e[i].iter() {
        if j != p {
            dfs(j, i, e, dp);
            dp[i][0] *= dp[j][0] + dp[j][1] % MOD;
            dp[i][1] *= dp[j][0] % MOD;
            dp[i][0] %= MOD;
            dp[i][1] %= MOD;
        }
    }
}
