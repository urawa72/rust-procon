use proconio::{fastout, input};

#[fastout]
fn main() {
    input! { s3: [String; 3] };

    for s in &["ABC", "ARC", "AGC", "AHC"] {
        if !s3.contains(&s.to_string()) {
            println!("{}", s);
        }
    }
}
