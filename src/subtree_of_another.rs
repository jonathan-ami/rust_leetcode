use crate::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
pub struct Solution {}
impl Solution {
    pub fn is_subtree(
        root: Option<Rc<RefCell<TreeNode>>>,
        sub_root: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        if sub_root.is_none() {
            return true;
        }
        if root.is_none() {
            return false;
        }
        if Self::is_same_tree(root.clone(), sub_root) {
            return true;
        }

        let node = root.unwrap().borrow();
        Self::is_subtree(node.right.clone(), sub_root.clone())
            || Self::is_subtree(node.left.clone(), sub_root.clone())
    }

    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        if p.is_none() && q.is_none() {
            return true;
        }

        if let (Some(n1), Some(n2)) = (p, q) {
            let node1 = n1.borrow();
            let node2 = n2.borrow();

            if node1.val != node2.val {
                return false;
            }
            return Self::is_same_tree(node1.left.clone(), node2.left.clone())
                && Self::is_same_tree(node1.right.clone(), node2.left.clone());
        }
        false
    }
}
