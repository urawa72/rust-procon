#[allow(unused_imports)]
use crate::util::linked_list::{to_list, ListNode};
pub struct Solution {}

#[allow(unused_assignments)]
// problem: https://leetcode.com/problems/remove-duplicates-from-sorted-list/submissions/
impl Solution {
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut l = head.clone();
        let mut v = 0;
        match head {
            Some(n) => v = n.val,
            None => return None,
        }
        let mut a = Box::new(ListNode::new(v));
        let mut current = &mut a;
        let mut n = 0;
        while l.is_some() {
            n = 0;
            if let Some(node) = l {
                l = node.next;
                n = node.val;
            }
            if n != current.val {
                current.next = Some(Box::new(ListNode::new(n)));
                current = current.next.as_mut().unwrap();
            }
        }
        Some(a)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn s0083_case1() {
        assert_eq!(
            to_list(vec![1, 2, 3]),
            Solution::delete_duplicates(to_list(vec![1, 1, 2, 3, 3]))
        )
    }

    #[test]
    fn s0083_case2() {
        assert_eq!(
            to_list(vec![0, 2, 100]),
            Solution::delete_duplicates(to_list(vec![0, 0, 2, 2, 100, 100]))
        )
    }

    #[test]
    fn s0083_case3() {
        assert_eq!(None, Solution::delete_duplicates(to_list(vec![])))
    }
}
