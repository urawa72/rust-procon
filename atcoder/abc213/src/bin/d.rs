use itertools::Itertools;
use proconio::{fastout, input};

fn dfs(g: &Vec<Vec<usize>>, cur: usize, prev: usize, ans: &mut Vec<usize>) {
    ans.push(cur + 1);
    for &next in &g[cur] {
        if prev == next {
            continue;
        }
        dfs(g, next, cur, ans);
        ans.push(cur + 1);
    }
}

#[fastout]
fn main() {
    input! { n: usize }
    let mut g = vec![vec![]; n];
    for _ in 0..n - 1 {
        input! { mut a: usize, mut b: usize };
        g[a - 1].push(b - 1);
        g[b - 1].push(a - 1);
    }
    for i in 0..n {
        g[i].sort();
    }
    let mut ans = vec![];
    dfs(&g, 0, n, &mut ans);
    println!("{}", ans.iter().join(" "));
}
