use crate::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
pub struct Solution {}

impl Solution {
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn height(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
            if let Some(node) = root {
                let node = node.borrow();
                let lh = height(node.left.clone());
                if lh == -1 {
                    return -1;
                }
                let rh = height(node.right.clone());
                if rh == -1 {
                    return -1;
                }
                if (lh - rh).abs() > 1 {
                    return -1;
                }
                return std::cmp::max(lh, rh) + 1;
            }
            0
        }
        if height(root) != -1 {
            return true;
        }
        false
    }
}
