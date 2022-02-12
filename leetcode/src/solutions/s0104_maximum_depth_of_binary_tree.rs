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
pub struct Solution {}
use crate::util::tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

// problem: https://leetcode.com/problems/maximum-depth-of-binary-tree/
impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        dfs(&root, 0)
    }
}

fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, val: i32) -> i32 {
    match node {
        Some(no) => {
            let no = no.borrow();
            let l = dfs(&no.left, val + 1);
            let r = dfs(&no.right, val + 1);
            std::cmp::max(l, r)
        }
        None => val,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::tree::to_tree;

    #[test]
    fn s0104_case1() {
        let tree = to_tree(vec![
            Some(3),
            Some(9),
            Some(20),
            None,
            None,
            Some(15),
            Some(7),
        ]);
        assert_eq!(Solution::max_depth(tree), 3);
    }

    #[test]
    fn s0104_case2() {
        let tree = to_tree(vec![Some(1), None, Some(2)]);
        assert_eq!(Solution::max_depth(tree), 2);
    }
}
