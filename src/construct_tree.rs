use crate::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
pub struct Solution {}
impl Solution {
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        if preorder.is_empty() || inorder.is_empty() {
            return None;
        }

        let mid = inorder.iter().position(|&x| x == preorder[0]).unwrap();
        let root_node = Rc::new(RefCell::new(TreeNode::new(preorder[0])));
        let left = Self::build_tree(preorder[1..=mid].to_vec(), inorder[..mid].to_vec());
        let right = Self::build_tree(preorder[mid + 1..].to_vec(), inorder[mid..].to_vec());
        root_node.borrow_mut().left = left;
        root_node.borrow_mut().right = right;
        Some(root_node)
    }
}
