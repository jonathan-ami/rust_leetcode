use crate::ListNode;
pub struct Solution {}
impl Solution {
    pub fn merge_two_lists(
        mut list1: Option<Box<ListNode>>,
        mut list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut dummy_head = Some(Box::new(ListNode::new(0)));
        let mut tail = &mut dummy_head;

        let (mut list1, mut list2) = (list1, list2);

        while let (Some(mut node1), Some(mut node2)) = (list1.as_mut(), list2.as_mut()) {
            if node1.val < node2.val {
                let next = node1.next.take();
                tail.as_mut().unwrap().next = list1;
                list1 = next;
            } else {
                let next = node2.next.take();
                tail.as_mut().unwrap().next = list2;
                list2 = next;
            }
            tail = &mut tail.as_mut().unwrap().next;
        }

        if list1.is_some() {
            tail.as_mut().unwrap().next = list1;
        } else {
            tail.as_mut().unwrap().next = list2;
        }

        dummy_head.unwrap().next
    }
}
