// use std::collections::HashSet;
// use im_rc::HashMap;
use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! { _: usize, _: usize, n: usize, ab: [(usize, usize); n] }
    // let hh = ab.iter().map(|a| a.0).sorted().collect::<HashSet<_>>();
    // let hh = hh.iter().sorted().zip(0..).collect::<HashMap<_, _>>();
    // let ww = ab.iter().map(|a| a.1).sorted().collect::<HashSet<_>>();
    // let ww = ww.iter().sorted().zip(0..).collect::<HashMap<_, _>>();
    // for a in ab {
    //     println!("{} {}", hh[&a.0] + 1, ww[&a.1] + 1);
    // }

    let mut h = ab.iter().map(|e| e.0).collect_vec();
    let mut w = ab.iter().map(|e| e.1).collect_vec();
    h.sort();
    h.dedup();
    w.sort();
    w.dedup();
    for &(a, b) in &ab {
        println!(
            "{} {}",
            h.binary_search(&a).unwrap() + 1,
            w.binary_search(&b).unwrap() + 1
        );
    }
}
