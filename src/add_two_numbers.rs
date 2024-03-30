use crate::ListNode;
pub struct Solution {}
impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode::new(0));
        let mut l1 = l1.clone();
        let mut l2 = l2.clone();

        let mut cur = &mut dummy;
        let mut carry = 0;
        while l1.is_some() || l2.is_some() || carry != 0 {
            let mut val1 = 0;
            let mut val2 = 0;
            if let Some(node) = l1 {
                val1 = node.val;
                l1 = node.next;
            }

            if let Some(node) = l2 {
                val2 = node.val;
                l2 = node.next;
            }

            cur.next = Some(Box::new(ListNode::new((val1 + val2 + carry) % 10)));
            cur = cur.next.as_mut().unwrap();
            carry = (val1 + val2 + carry) / 10;
        }

        dummy.next
    }
}
