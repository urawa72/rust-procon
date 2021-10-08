use proconio::{input, fastout};

#[fastout]
fn main() {
    input! { mut p: usize }

    fn f(mut n: usize) -> usize {
        let mut res = 1;
        while n != 0 {
            res *= n;
            n -= 1;
        }
        res
    }

    let mut ans = 0;
    for i in (1..=10).rev() {
        let q = f(i);
        ans += p / q;
        p %= q;
    }
    println!("{}", ans);
}
