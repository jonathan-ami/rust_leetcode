use crate::TreeNode;
use ::std::collections::VecDeque;
use std::cell::RefCell;
use std::rc::Rc;
pub struct Solution {}
impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut queue = VecDeque::new();
        let mut res: Vec<Vec<i32>> = Vec::new();
        queue.push_back(root);
        while !queue.is_empty() {
            let len = queue.len();
            let mut level: Vec<i32> = Vec::new();
            for _i in 0..len {
                if let Some(node) = queue.pop_front().unwrap() {
                    let n = node.borrow();
                    level.push(n.val.clone());
                    queue.push_back(n.left.clone());
                    queue.push_back(n.right.clone());
                }
            }
            if level.is
            res.push(level)
        }
        res
    }
}
