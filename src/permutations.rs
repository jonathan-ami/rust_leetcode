pub struct Solution {}
impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        use std::collections::VecDeque;
        let mut deque: VecDeque<i32> = VecDeque::from(nums);
        fn rec(mut nums: VecDeque<i32>) -> Vec<Vec<i32>> {
            let mut r = Vec::new();
            if nums.len() == 1 {
                let mut res = Vec::new();
                res.push(nums.clone().into());
                return res;
            }
            for i in 0..nums.len() {
                let n = nums.pop_front().unwrap();
                let mut perms = rec(nums.clone());
                for perm in &mut perms {
                    perm.push(n);
                }
                nums.push_back(n);
                r.extend(perms.clone());
            }

            return r;
        }
        let mut res = Vec::new();
        res.extend(rec(deque));
        res
    }
}
