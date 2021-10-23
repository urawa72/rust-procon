use proconio::{fastout, input};

#[fastout]
fn main() {
    input! { n: usize, ab: [(f64, f64); n] }

    let mut sum = 0.0;
    for i in ab.iter() {
        sum += i.0 / i.1;
    }
    sum /= 2.0;
    let mut ans = 0.0;
    for i in ab.iter() {
        let tmp = i.0 / i.1;
        if tmp <= sum {
            sum -= tmp;
            ans += i.0;
        } else {
            ans += i.1 * sum;
            break;
        }
    }
    println!("{}", ans);
}
