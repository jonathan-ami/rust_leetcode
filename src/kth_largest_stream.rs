use std::cmp::Reverse;
use std::collections::BinaryHeap;
struct KthLargest {
    nums: Vec<i32>,
    k: i32,
    heap: BinaryHeap<Reverse<i32>>,
}
impl KthLargest {
    fn new(k: i32, nums: Vec<i32>) -> Self {
        let mut heap = BinaryHeap::new();
        for val in nums {
            heap.push(Reverse(val));
            if heap.len() > k as usize {
                heap.pop();
            }
        }
        println!("{:?}", heap);
        KthLargest { nums, k, heap }
    }
    fn add(&mut self, val: i32) -> i32 {
        self.heap.push(Reverse(val));
        self.heap.pop().unwrap();
        self.heap.peek().unwrap().0
    }
}
