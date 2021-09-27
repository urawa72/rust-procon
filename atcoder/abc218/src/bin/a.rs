use proconio::{fastout, input};

#[fastout]
fn main() {
  input! { n: usize, s: String };
  if s.chars().nth(n - 1).unwrap() == 'o' {
      println!("Yes");
  } else {
    println!("No");
  }
}
