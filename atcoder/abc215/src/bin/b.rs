use proconio::{fastout, input};

#[fastout]
fn main() {
    input! { n: usize }
    let mut m = 1;
    let mut ans = 0;
    loop {
        m *= 2;
        if m <= n {
            ans += 1;
        } else {
            break;
        }
    }
    println!("{}", ans);
}
