use proconio::{input, fastout};

#[fastout]
fn main() {
    input! { h: usize, w: usize, t: [[usize; w]; h] }

    for i1 in 0..h {
        for j1 in 0..w {
            for i2 in i1 + 1..h {
                for j2 in j1 + 1..w {
                    if t[i1][j1] + t[i2][j2] <= t[i2][j1] + t[i1][j2] {
                        continue;
                    } else {
                        println!("No");
                        return;
                    }
                }
            }
        }
    }
    println!("Yes");
}
