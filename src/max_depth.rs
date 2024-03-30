use crate::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
pub struct Solution {}
impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(node) = root {
            let node = node.borrow();
            1 + std::cmp::max(
                Self::max_depth(node.left.clone()),
                Self::max_depth(node.right.clone()),
            ) as i32
        } else {
            0
        }
    }
}
