use crate::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
pub struct Solution {}
impl Solution {
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut res = 0;

        fn dfs(root: Option<Rc<RefCell<TreeNode>>>, res: &mut i32) -> i32 {
            if let Some(node) = root {
                let node = node.borrow();
                let left = dfs(node.left.clone(), res);
                let right = dfs(node.right.clone(), res);
                *res = std::cmp::max(*res, 2 + left + right);
                return 1 + std::cmp::max(left, right);
            }
            -1
        }
        dfs(root, &mut res);
        res
    }
}
