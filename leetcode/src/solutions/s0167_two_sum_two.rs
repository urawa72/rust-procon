struct Solution;

impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        for (i, n) in numbers.iter().enumerate() {
            let diff = target - n;
            if let Some(j) = Solution::find_index(&numbers, diff, i) {
                let i1 = i as i32 + 1;
                let i2 = (j + 1) as i32;
                let mut ans = vec![i1, i2];
                ans.sort();
                return ans;
            }
        }
        vec![]
    }

    fn find_index(numbers: &[i32], find: i32, find_idx: usize) -> Option<usize> {
        let len = numbers.len();
        let mut m = len / 2;
        let mut r = len - 1;
        let mut l = 0;
        let mut cur = numbers[m];

        while l <= r {
            match cur.cmp(&find) {
                std::cmp::Ordering::Equal if m != find_idx => return Some(m),
                std::cmp::Ordering::Equal => return None,
                std::cmp::Ordering::Less => l = m + 1,
                std::cmp::Ordering::Greater => r = m - 1,
            }
            m = (r + l) / 2;
            cur = numbers[m];
        }
        None
    }
}
