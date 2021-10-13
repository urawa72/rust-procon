use proconio::{fastout, input};

#[fastout]
fn main() {
    input! { n: usize, p: usize, a: [usize; n] }
    let ans = a.iter().filter(|&x| x < &p).collect::<Vec<_>>().len();
    println!("{}", ans);
}
