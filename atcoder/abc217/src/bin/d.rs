use std::collections::BTreeSet;

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! { l: usize, q: usize, query: [(usize, usize); q] }
    let mut bts = BTreeSet::new();
    bts.insert(0);
    bts.insert(l);

    for (c, x) in query {
        if c == 1 {
            bts.insert(x);
        } else {
            let l = bts.range(..x).next_back().unwrap();
            let r = bts.range(x..).next().unwrap();
            println!("{}", *r - *l);
        }
    }
}
