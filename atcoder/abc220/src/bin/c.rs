use proconio::{input, fastout};

#[fastout]
fn main() {
    input! { n: usize, a: [usize; n], x: usize }
    let sum: usize = a.iter().sum();
    let y = x / sum;
    let z = x % sum;
    let mut c = 0;
    let mut sum = 0;
    for i in a {
        sum += i;
        c += 1;
        if z < sum {
            break;
        }
    }
    println!("{}", y * n + c);
}
