use crate::ListNode;
pub struct Solution {}
impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode::new(0));
        dummy.next = head;
        let mut cur = dummy.clone();
        let mut cur2 = dummy.as_mut();
        let mut count = 0;
        while cur.next.is_some() {
            count += 1;
            if count > n {
                cur2 = cur2.next.as_mut().unwrap();
            }
            cur = cur.next.unwrap();
        }

        let last = cur2.next.as_mut().unwrap();
        cur2.next = last.next.clone();
        dummy.next
    }
}
