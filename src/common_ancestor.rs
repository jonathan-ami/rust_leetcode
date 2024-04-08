use crate::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
pub struct Solution {}
impl Solution {
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let mut cur = root;

        let q = q.unwrap();
        let q = q.borrow();

        let p = p.unwrap();
        let p = p.borrow();
        while let Some(node) = cur {
            let val = node.borrow().val;
            if p.val < val && q.val < val {
                cur = node.borrow().left.clone();
            } else if p.val > val && p.val < val {
                cur = node.borrow().right.clone();
            } else {
                return Some(node);
            }
        }
        None
    }
}
