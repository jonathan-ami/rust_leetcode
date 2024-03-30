use crate::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
pub struct Solution {}
impl Solution {
    pub fn invert_tree(
        &self,
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if root.is_none() {
            return None;
        }

        let parent = root;
        let left = parent.clone().unwrap().into_inner().left;
        let right = parent.clone().unwrap().into_inner().right;
        if let (Some(left), Some(right)) = (left, right) {
            left.swap(&right);
        }

        let _ = self.invert_tree(parent.clone().unwrap().into_inner().left);
        let _ = self.invert_tree(parent.clone().unwrap().into_inner().right);
        parent
    }
}
