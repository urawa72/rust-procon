use proconio::{fastout, input};

#[allow(unused_macros)]
macro_rules ! min {($ a : expr $ (, ) * ) => {{$ a } } ; ($ a : expr , $ b : expr $ (, ) * ) => {{std :: cmp :: min ($ a , $ b ) } } ; ($ a : expr , $ ($ rest : expr ) ,+ $ (, ) * ) => {{std :: cmp :: min ($ a , min ! ($ ($ rest ) ,+ ) ) } } ; }
#[allow(unused_macros)]
macro_rules ! chmin {($ base : expr , $ ($ cmps : expr ) ,+ $ (, ) * ) => {{let cmp_min = min ! ($ ($ cmps ) ,+ ) ; if $ base > cmp_min {$ base = cmp_min ; true } else {false } } } ; }

const INF: usize = 10000000000;
const MAX_V: usize = 100100;

#[fastout]
fn main() {
    input! { n: usize, w: usize, v: [(usize, usize); n] }

    // INFで初期化
    let mut dp = vec![vec![INF; MAX_V + 2000]; n + 10];

    // 初期値をセット
    dp[0][0] = 0;

    // 配るDP
    for i in 0..n {
        for j in 0..=MAX_V {
            // 選ばない
            chmin!(dp[i + 1][j], dp[i][j]);
            // 選ぶ
            chmin!(dp[i + 1][j + v[i].1], dp[i][j] + v[i].0);
        }
    }

    // w以下で探す
    let mut ans = 0;
    for i in 0..=MAX_V {
        if dp[n][i] <= w {
            ans = i;
        }
    }
    println!("{}", ans);
}
