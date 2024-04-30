use std::cmp::Reverse;
use std::collections::BinaryHeap;
pub struct Solution {}
impl Solution {
    pub fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let mut heap: BinaryHeap<Reverse<(i32, Vec<i32>)>> = BinaryHeap::new();
        for point in points {
            heap.push(Reverse((
                (point[0] * point[0]) + (point[1] * point[1]),
                point,
            )));
        }
        let mut res: Vec<Vec<i32>> = Vec::new();
        for _ in 0..k {
            res.push(heap.pop().unwrap().0 .1);
        }
        res
    }
}
