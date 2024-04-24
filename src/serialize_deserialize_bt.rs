use crate::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
pub struct Codec {}
impl Codec {
    fn new() -> Self {}
    fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        self.preorder(root)
    }
    fn preorder(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        let mut res = String::new();
        if let Some(node) = root {
            let node = node.borrow();
            res.push_str(&node.val.to_string());
            res.push(',');
            res.push_str(&self.preorder(node.left.clone()));
            res.push_str(&self.preorder(node.right.clone()));
            return res;
        }
        "null,".to_string()
    }

    fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        let parts: Vec<&str> = data.split(',').collect();
        println!("{:?}", parts);
        fn rec(parts: &Vec<&str>, i: &mut usize) -> Option<Rc<RefCell<TreeNode>>> {
            let val = parts[*i];
            if parts[*i] == "null" {
                *i += 1;
                return None;
            }
            let mut node = Rc::new(RefCell::new(TreeNode::new(val.parse().unwrap())));
            *i += 1;
            let left = rec(parts, i);
            let right = rec(parts, i);
            node.borrow_mut().left = left;
            node.borrow_mut().right = right;
            Some(node)
        }
        rec(&parts, &mut 0)
    }
}
