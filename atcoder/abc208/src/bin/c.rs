// use std::collections::BTreeMap;

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! { n: usize, mut k: usize, a: [usize; n] }
    // let mut btm = BTreeMap::new();
    // let m = k / n;
    // k %= n;
    // for i in 0..n {
    //     btm.entry(a[i]).or_insert(m);
    // }
    // if 0 < k {
    //     for (_, value) in btm.iter_mut() {
    //         *value += 1;
    //         k -= 1;
    //         if k == 0 {
    //             break;
    //         }
    //     }
    // }

    // for i in 0..n {
    //     println!("{}", btm.entry(a[i]).or_insert(0));
    // }

    let mut b = a.clone();
    b.sort();
    for i in a {
        if i < b[k % n] {
            println!("{}", (k + n - 1) / n);
        } else {
            println!("{}", k / n);
        }
    }
}
