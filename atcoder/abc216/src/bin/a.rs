use proconio::{fastout, input};

#[fastout]
fn main() {
    input! { s: String }
    let xy = s.split('.').collect::<Vec<_>>();
    let x = xy[0];
    let y: usize = xy[1].parse().unwrap();
    let ans = match y {
        0..=2 => x.to_string() + "-",
        3..=6 => x.to_string(),
        _ => x.to_string() + "+"
    };
    println!("{}", ans);
}
