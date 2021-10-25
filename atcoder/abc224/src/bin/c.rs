use proconio::{fastout, input};

#[fastout]
fn main() {
    input! { n: usize, p: [(isize, isize); n] }

    // 座標上の3点から三角形を求める公式
    // 三角形であるかだけ判定すれば良いので、abs()と2で割るのは不要
    // 0の場合は成立しない
    let mut ans = 0;
    for i in 0..n {
        for j in i + 1..n {
            for k in j + 1..n {
                if ((p[j].0 - p[i].0) * (p[k].1 - p[i].1) - (p[k].0 - p[i].0) * (p[j].1 - p[i].1))
                    != 0
                {
                    ans += 1;
                }
            }
        }
    }
    println!("{}", ans);
}
