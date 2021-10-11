use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! { n: Chars }

    let mut ans = 1;
    for bit in 0..(1 << n.len()) {
        let mut tmp1 = vec![];
        let mut tmp2 = vec![];
        for i in 0..n.len() {
            if (bit & (1 << i)) != 0 {
                tmp1.push(n[i]);
            } else {
                tmp2.push(n[i]);
            }
        }
        tmp1.sort();
        tmp2.sort();
        if tmp1.is_empty() || tmp2.is_empty() {
            continue;
        }
        let x: usize = tmp1.iter().rev().collect::<String>().parse().unwrap();
        let y: usize = tmp2.iter().rev().collect::<String>().parse().unwrap();
        ans = std::cmp::max(ans, x * y);
    }
    println!("{}", ans);
}
