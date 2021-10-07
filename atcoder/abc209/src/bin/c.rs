use proconio::{input, fastout};

#[fastout]
fn main() {
    input! { n: usize, mut c: [usize; n] }
    c.sort();
    let mut ans = 1;
    for i in 0..n {
        ans *= std::cmp::max(0, c[i] - i);
        ans %= 1000000007;
    }
    println!("{}", ans);
}
