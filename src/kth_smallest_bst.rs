use crate::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
pub struct Solution {}
impl Solution {
    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        let mut stack: Vec<Rc<RefCell<TreeNode>>> = Vec::new();
        let mut count = 0;
        let mut cur = root.clone();
        while cur.is_some() || !stack.is_empty() {
            while let Some(node) = cur {
                stack.push(node.clone());
                let node = node.borrow();
                cur = node.left.clone();
            }
            cur = stack.pop();
            count += 1;
            let node = cur.unwrap();
            let node = node.borrow();
            if count == k {
                return node.val;
            }
            cur = node.right.clone();
        }
        count
    }
}
