pub struct Solution {}
impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        use std::collections::VecDeque;
        let mut result: Vec<i32> = Vec::new();
        let mut max: i32 = 0;
        let mut deque: VecDeque<usize> = VecDeque::new();
        let mut lp = 0;
        let mut rp: usize = 0;
        while rp < nums.len() {
            while !deque.is_empty() && nums[rp] > nums[*deque.front().unwrap()] {
                deque.pop_front();
            }
            deque.push_front(rp);
            if !deque.is_empty() && lp > *deque.back().unwrap() {
                deque.pop_back();
            }
            if (rp + 1) as i32 >= k {
                result.push(nums[*deque.back().unwrap()]);
                lp += 1;
            }
            rp += 1;
        }
        result
    }
}
