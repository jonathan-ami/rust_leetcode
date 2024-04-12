use crate::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
pub struct Solution {}
impl Solution {
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        use std::collections::VecDeque;
        let mut q: VecDeque<Option<Rc<RefCell<TreeNode>>>> = VecDeque::new();
        let mut res: Vec<i32> = Vec::new();
        q.push_back(root);
        while !q.is_empty() {
            let len = q.len();
            let mut right = 0;
            for _i in 0..len {
                if let Some(node) = q.pop_front().unwrap() {
                    let n = node.borrow();
                    right = n.val;
                    q.push_back(n.left.clone());
                    q.push_back(n.right.clone());
                }
            }
            res.push(right);
        }

        res
    }
}
