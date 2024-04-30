use std::collections::BinaryHeap;
pub struct Solution {}
impl Solution {
    pub fn last_stone_weight(stones: Vec<i32>) -> i32 {
        let mut heap = BinaryHeap::new();
        for stone in stones {
            heap.push(stone);
        }
        while !heap.is_empty() {
            let max = heap.pop().unwrap();
            if let Some(second_max) = heap.pop() {
                let res = max - second_max;
                if res != 0 {
                    heap.push(res);
                }
            } else {
                return max;
            }
        }
        0
    }
}
