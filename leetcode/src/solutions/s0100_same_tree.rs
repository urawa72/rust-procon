// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
#[allow(unused_imports)]
pub struct Solution;
use crate::util::tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        match (p, q) {
            (None, None) => true,
            (Some(p), Some(q)) => {
                let p = p.borrow();
                let q = q.borrow();
                p.val == q.val
                    && Self::is_same_tree(p.left.clone(), q.left.clone())
                    && Self::is_same_tree(p.right.clone(), q.right.clone())
            }
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::tree::to_tree;

    #[test]
    fn s0100_case1() {
        let tree1 = to_tree(vec![Some(1), Some(2), Some(3)]);
        let tree2 = to_tree(vec![Some(1), Some(2), Some(3)]);
        assert_eq!(Solution::is_same_tree(tree1, tree2), true);
    }

    #[test]
    fn s0100_case2() {
        let tree1 = to_tree(vec![Some(1), Some(2)]);
        let tree2 = to_tree(vec![Some(1), None, Some(2)]);
        assert_eq!(Solution::is_same_tree(tree1, tree2), false);
    }

    #[test]
    fn s0100_case3() {
        let tree1 = to_tree(vec![
            Some(3),
            Some(9),
            Some(20),
            None,
            None,
            Some(15),
            Some(7),
        ]);
        let tree2 = to_tree(vec![
            Some(3),
            Some(9),
            Some(20),
            None,
            None,
            Some(15),
            Some(7),
        ]);
        assert_eq!(Solution::is_same_tree(tree1, tree2), true);
    }
}
