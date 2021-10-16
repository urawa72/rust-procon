use std::collections::BTreeMap;

use proconio::{input, fastout};

#[fastout]
fn main() {
    input! { n: usize, mut k: usize, mut ab: [(usize, usize); n] }
    let mut map = BTreeMap::new();
    for (a, b) in &ab {
        *map.entry(a).or_insert(0) += b;
    }
    let mut ans = 0;
    let mut pre = 0;
    for (a, b) in map {
        if ans + k < *a {
            ans += k;
            k = 0;
            break;
        }
        ans = *a;
        k = k - (a - pre) + b;
        pre = *a;
    }
    println!("{}", ans + k);
}
