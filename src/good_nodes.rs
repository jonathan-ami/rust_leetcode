use crate::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
pub struct Solution {}
impl Solution {
    pub fn good_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn rec(root: Option<Rc<RefCell<TreeNode>>>, mut max: i32) -> i32 {
            if let Some(node) = root {
                let node = node.borrow();
                let mut count = 0;
                if node.val == max {
                    count = 1;
                } else if node.val > max {
                    count = 1;
                    max = node.val;
                }
                return count + rec(node.left.clone(), max) + rec(node.right.clone(), max);
            }
            0
        }
        rec(root, i32::MIN)
    }
}
