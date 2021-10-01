use proconio::{fastout, input};
use std::collections::HashSet;

fn factorize(n: i32) -> Vec<i32> {
    let mut i = 1;
    let mut v = vec![];
    while i * i <= n {
        if n % i == 0 {
            v.push(i);
            v.push(n / i);
        }
        i += 1;
    }
    v
}

#[fastout]
fn main() {
    input! { n: i32, m: i32, a: [i32; n] }

    let hs = (0..n as usize)
        .into_iter()
        .flat_map(|i| factorize(a[i]))
        .collect::<HashSet<i32>>();
    let ans = (1..(m + 1))
        .filter(|&i| {
            let tmp = factorize(i).into_iter().collect::<HashSet<i32>>();
            tmp.intersection(&hs).count() == 1
        })
        .collect::<Vec<i32>>();

    println!("{}", ans.len());
    println!(
        "{}",
        ans.into_iter()
            .map(|i| i.to_string())
            .collect::<Vec<String>>()
            .join("\n")
    )
}
