use crate::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
pub struct Solution {}
impl Solution {
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        if p.is_none() && q.is_none() {
            return true;
        }
        if let (Some(node), Some(node2)) = (p, q) {
            let node1 = node.borrow();
            let node2 = node2.borrow();

            if node1.val != node2.val {
                return false;
            }

            return Self::is_same_tree(node1.left.clone(), node2.left.clone())
                && Self::is_same_tree(node1.right.clone(), node2.right.clone());
        }
        false
    }
}
