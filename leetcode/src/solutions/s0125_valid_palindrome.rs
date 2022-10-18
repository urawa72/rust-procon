struct Solution;

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let s = s.to_lowercase();
        let s: String = s
            .chars()
            .into_iter()
            .filter(|c| ('a'..='z').contains(c) || ('0'..='9').contains(c))
            .collect();

        let t: String = s.chars().rev().collect();
        s == t
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn case1() {
        assert!(Solution::is_palindrome(
            "A man, a plan, a canal: Panama".to_string()
        ));
    }

    #[test]
    fn case2() {
        assert!(!Solution::is_palindrome("race a car".to_string()));
    }

    #[test]
    fn case3() {
        assert!(Solution::is_palindrome("".to_string()));
    }

    #[test]
    fn case4() {
        assert!(!Solution::is_palindrome("0P".to_string()));
    }
}
