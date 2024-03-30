use crate::ListNode;
pub struct Solution {}
impl Solution {
    pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
        let mut rp = head.clone();
        let mut lp = head.clone();
        while rp.is_some() && rp.as_ref().unwrap().next.is_some() {
            rp = rp.as_ref().unwrap().next.as_ref().unwrap().next:bn;
        }
    }
}
