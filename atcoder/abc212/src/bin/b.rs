use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! { x: Chars }
    if x[0] == x[1] && x[1] == x[2] && x[2] == x[3] {
        println!("Weak");
        return;
    }

    let mut flag = false;
    for i in 0..3 {
        if x[i] == '9' {
            if x[i + 1] != '0' {
                flag = true;
                break;
            }
        } else {
            let a = x[i] as i32 - 48;
            let b = x[i + 1] as i32 - 48;
            if a + 1 != b {
                flag = true;
                break;
            }
        }
    }
    println!("{}", if flag { "Strong" } else { "Weak" })
}
