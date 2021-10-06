use proconio::{fastout, input};

#[fastout]
fn main() {
    input! { n: usize, mut x: usize, a: [usize; n] }

    let mut c = 0;
    for i in 0..n {
        if a[i] <= x {
            if i % 2 == 0 {
                x -= a[i] - 1;
            } else {
                x -= a[i];
            }
            c += 1;
        } else {
            break;
        }
    }

    println!("{}", if c == n { "Yes" } else { "No" });
}
