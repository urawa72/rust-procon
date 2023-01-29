use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut map: HashMap<String, Vec<String>> = HashMap::new();

        for s in strs {
            let mut t = s.chars().collect::<Vec<_>>();
            t.sort();
            let t: String = t.into_iter().collect();
            map.entry(t).or_default().push(s);
        }

        map.into_values().collect()
    }
}
