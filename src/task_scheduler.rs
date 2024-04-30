use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, VecDeque};
pub struct Solution {}
impl Solution {
    pub fn least_interval(tasks: Vec<char>, n: i32) -> i32 {
        let mut count: HashMap<char, i32> = HashMap::new();
        let mut heap: BinaryHeap<i32> = BinaryHeap::new();
        let mut queue: VecDeque<(i32, i32)> = VecDeque::new();
        let mut time = 0;
        for val in tasks {
            *count.entry(val).or_insert(0) += 1;
        }
        for val in count {
            heap.push(val.1);
        }

        while !heap.is_empty() || !queue.is_empty() {
            time += 1;
            if let Some(mut val) = heap.pop() {
                val -= 1;
                if val > 0 {
                    queue.push_back((val, time + n));
                }
            }

            if !queue.is_empty() && queue.front().unwrap().1 == time {
                heap.push(queue.pop_front().unwrap().0);
            }
        }
        time
    }
}
