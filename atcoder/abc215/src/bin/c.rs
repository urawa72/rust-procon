use permutohedron::LexicalPermutation;
// use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! { s: String, k: usize }
    let mut s = s.chars().collect::<Vec<char>>();
    s.sort();
    // let ans: Vec<_> = s.iter().permutations(s.len()).unique().nth(k - 1).unwrap();
    // println!("{}", ans.iter().map(|c| *c).collect::<String>());
    for _ in 0..k - 1 {
        s.next_permutation();
    }
    println!("{}", s.iter().collect::<String>());
}
