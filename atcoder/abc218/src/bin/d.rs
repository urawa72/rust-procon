use proconio::{input, fastout};
use std::collections::HashSet;

#[fastout]
fn main() {
    input! { n: usize, xy: [(usize, usize); n] };

    let hs = xy.iter().map(|&e| e).collect::<HashSet<_>>();

    let mut ans = 0;
    for i in 0..n {
        for j in 0..n {
            if xy[i].0 < xy[j].0 && xy[i].1 < xy[j].1 {
                if hs.contains(&(xy[i].0, xy[j].1)) && hs.contains(&(xy[j].0, xy[i].1)) {
                    ans += 1;
                }
            }
        }
    }
    println!("{}", ans);
}
