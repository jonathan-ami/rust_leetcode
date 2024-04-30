use std::cmp::Reverse;
use std::collections::BinaryHeap;
pub struct Solution {}
impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let mut heap: BinaryHeap<Reverse<i32>> = BinaryHeap::new();
        for val in nums {
            if heap.len() < k as usize {
                heap.push(Reverse(val));
            } else {
                if val > heap.peek().unwrap().0 {
                    heap.pop();
                    heap.push(Reverse(val));
                }
            }
        }
        heap.pop().unwrap().0
    }
}
