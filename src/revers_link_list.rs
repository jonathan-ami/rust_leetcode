use crate::ListNode;
pub struct Solution {}
impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        recursion(head);
        head
    }
    pub fn recursion(&self, cur: Option<Box<ListNode>>) {
        let mut cur = head;
        let mut prev = None;
        while let Some(mut node) = cur {
            let next = node.next.take();
            node.next = prev;
            prev = Some(node);
            cur = next;
        }
        prev
    }
}
