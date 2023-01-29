pub struct Solution {}

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut s = s.chars().collect::<Vec<_>>();
        s.sort();
        let mut t = t.chars().collect::<Vec<_>>();
        t.sort();

        s == t
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        assert!(Solution::is_anagram(
            "anagram".to_string(),
            "nagaram".to_string()
        ))
    }
    #[test]
    fn case2() {
        assert!(!Solution::is_anagram("rat".to_string(), "cat".to_string()))
    }
}
