
use libm::pow;
use proconio::{input, fastout};

#[fastout]
fn main() {
    input! { k: f64, a: String, b: String }
    let mut e = 0 as f64;
    let mut x = 0;
    for c in a.chars().rev() {
        x += pow(k, e) as i64 * (c as i64 - 48);
        e += 1.0;
    }
    let mut y = 0;
    let mut e = 0.0;
    for c in b.chars().rev() {
        y += pow(k, e) as i64 * (c as i64 - 48);
        e += 1.0;
    }
    println!("{}", x * y);

}
