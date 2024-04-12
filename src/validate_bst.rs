use crate::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
pub struct Solution {}
impl Solution {
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn rec(root: Option<Rc<RefCell<TreeNode>>>, min: i64, max: i64) -> bool {
            if let Some(node) = root {
                let node = node.borrow();
                if (node.val as i64) > min && (node.val as i64) < max {
                    return rec(node.left.clone(), min, node.val.into())
                        && rec(node.right.clone(), node.val.into(), max);
                } else {
                    return false;
                }
            }
            true
        }
        rec(root, i64::MIN, i64::MAX)
    }
}
