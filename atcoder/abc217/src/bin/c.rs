use proconio::{input, fastout};

#[fastout]
fn main() {
    input! { n: usize, p: [usize; n] }

    let mut q = vec![0; n];
    for i in 0..n {
        q[p[i] - 1] = i + 1;
    }
    let v = q.iter().map(|v| v.to_string()).collect::<Vec<_>>();
    println!("{}", v.join(" "));
}
