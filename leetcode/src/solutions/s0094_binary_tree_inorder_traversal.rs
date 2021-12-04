#[allow(unused_imports)]
pub struct Solution {}
use crate::util::tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = vec![];

        fn traversal(node: Option<Rc<RefCell<TreeNode>>>, res: &mut Vec<i32>) {
            if let Some(n) = node {
                traversal(n.borrow().left.clone(), res);
                res.push(n.borrow().val);
                traversal(n.borrow().right.clone(), res);
            }
        }

        traversal(root, &mut res);
        res
    }
}

#[cfg(test)]
mod tests {
//    use super::*;

//    #[test]
//    fn s0094_case1() {
//        assert_eq!(
//            Solution::inorder_traversal(tree![1, null, 2, 3]),
//            vec![1, 3, 2]
//        );
//    }
}
