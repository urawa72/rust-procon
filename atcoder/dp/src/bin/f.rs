use proconio::{fastout, input, marker::Chars};

pub trait ExtendString {
    fn reverse(&self) -> String;
}
impl ExtendString for String {
    fn reverse(&self) -> String {
        self.chars().rev().collect::<String>()
    }
}

#[allow(unused_macros)]
macro_rules ! max {($ a : expr $ (, ) * ) => {{$ a } } ; ($ a : expr , $ b : expr $ (, ) * ) => {{std :: cmp :: max ($ a , $ b ) } } ; ($ a : expr , $ ($ rest : expr ) ,+ $ (, ) * ) => {{std :: cmp :: max ($ a , max ! ($ ($ rest ) ,+ ) ) } } ; }
#[allow(unused_macros)]
macro_rules ! chmax {($ base : expr , $ ($ cmps : expr ) ,+ $ (, ) * ) => {{let cmp_max = max ! ($ ($ cmps ) ,+ ) ; if $ base < cmp_max {$ base = cmp_max ; true } else {false } } } ; }

#[fastout]
fn main() {
    input! { s: Chars, t: Chars }

    let mut dp = vec![vec![0usize; 3100]; 3100];
    let mut n = s.len();
    let mut m = t.len();

    for i in 0..n {
        for j in 0..m {
            if s[i] == t[j] {
                chmax!(dp[i + 1][j + 1], dp[i][j] + 1);
            }
            chmax!(dp[i + 1][j + 1], dp[i + 1][j]);
            chmax!(dp[i + 1][j + 1], dp[i][j + 1]);
        }
    }

    // 復元
    let mut ans = "".to_string();
    while 0 < n && 0 < m {
        // 上から
        if dp[n][m] == dp[n - 1][m] {
            n -= 1;
        // 左から
        } else if dp[n][m] == dp[n][m - 1] {
            m -= 1;
        // 左上から（s[n] == t[n]）
        } else {
            ans.push(s[n - 1]);
            n -= 1;
            m -= 1;
        }
    }
    println!("{}", ans.reverse());
}
