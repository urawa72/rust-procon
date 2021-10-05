use std::collections::HashMap;

use proconio::{input, fastout};

#[fastout]
fn main() {
    input! { n: usize, k: usize, c: [usize; n] }

    let mut map = HashMap::new();

    let mut ans = 0;
    let mut r = 0;
    let mut cnt = 0;
    for l in 0..=n - k {
        while r < n && cnt != k {
            *map.entry(c[r]).or_insert(0) += 1;
            r += 1;
            cnt += 1;
        }
        ans = std::cmp::max(ans, map.len());

        *map.entry(c[l]).or_insert(0) -= 1;
        if map[&c[l]] == 0 {
            map.remove(&c[l]);
        }
        cnt -= 1;
    }
    println!("{}", ans);
}
