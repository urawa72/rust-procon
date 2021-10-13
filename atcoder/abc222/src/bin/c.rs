use std::{collections::BTreeMap, iter::FromIterator};

use proconio::{input, fastout, marker::Chars};

#[fastout]
fn main() {
    input! { n: usize, m: usize, a: [Chars; n * 2] }

    let mut map = BTreeMap::new();
    for i in 0..n * 2 {
        map.entry(i).or_insert(0);
    }

    for i in 0..m {
        let mut tmp = Vec::from_iter(map.clone());
        tmp.sort_by(|&(_, a), &(_, b)| b.cmp(&a));
        for j in 0..n {
            let c = &tmp[j * 2];
            let d = &tmp[j * 2 + 1];
            let x = a[c.0][i];
            let y = a[d.0][i];
            if (x == 'G' && y == 'P') || (x == 'P' && y == 'C') || (x == 'C' && y == 'G'){
                *map.entry(d.0).or_insert(0) += 1;
            } else if x == y {
                continue;
            } else {
                *map.entry(c.0).or_insert(0) += 1;
            }
        }
    }
    let mut tmp = Vec::from_iter(&map);
    tmp.sort_by(|&(_, a), &(_, b)| b.cmp(&a));
    for ans in tmp.iter() {
        println!("{}", ans.0 + 1);
    }

}
