use proconio::{input, fastout};
use std::cmp::min;

use std::cmp::Ordering;
pub trait BinarySearch<T> {
    fn lower_bound(&self, x: &T) -> usize;
    fn upper_bound(&self, x: &T) -> usize;
}
 impl<T: Ord> BinarySearch<T> for [T] {
     fn lower_bound(&self, x: &T) -> usize {
         let mut low = 0;
         let mut high = self.len();
         while low != high {
             let mid = (low + high) / 2;
             match self[mid].cmp(x) {
                 Ordering::Less => low = mid + 1,
                 Ordering::Equal | Ordering::Greater => high = mid,
             }
         }
         low
     }
     fn upper_bound(&self, x: &T) -> usize {
         let mut low = 0;
         let mut high = self.len();
         while low != high {
             let mid = (low + high) / 2;
             match self[mid].cmp(x) {
                 Ordering::Less | Ordering::Equal => low = mid + 1,
                 Ordering::Greater => high = mid,
             }
         }
         low
     }
 }

#[fastout]
fn main() {
    input! { n: usize, m: usize, a: [i32; n], mut b: [i32; m] }
    b.sort();
    let mut ans = 1000000001;
    for x in a {
        let l = b.lower_bound(&x);
        if l < m {
            ans = min(ans, (b[l] - x).abs());
        }
        if 0 < l {
            ans = min(ans, (b[l - 1] - x).abs());
        }
    }
    println!("{}", ans);
}
