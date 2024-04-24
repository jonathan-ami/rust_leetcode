use crate::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
pub struct Solution {}
impl Solution {
    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn rec(root: Option<Rc<RefCell<TreeNode>>>, max: &mut i32) -> i32 {
            if let Some(node) = root {
                let node = node.borrow();
                let left = rec(node.left.clone(), max).max(0);
                let right = rec(node.right.clone(), max).max(0);
                let split = left + right + node.val;
                *max = std::cmp::max(*max, split);
                return node.val + std::cmp::max(left, right);
            }
            0
        }

        let mut max = i32::MIN;
        std::cmp::max(rec(root, &mut max), max)
    }
}
