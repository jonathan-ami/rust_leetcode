pub struct Solution {}
impl Solution {
    pub fn can_partition(nums: Vec<i32>) -> bool {
        use std::collections::HashSet;
        let mut target: i32 = nums.iter().sum();
        if target % 2 == 0 {
            target = target / 2;
        } else {
            return false;
        }
        let mut mem: HashSet<i32> = HashSet::new();
        mem.insert(0);
        for i in (0..nums.len()).rev() {
            if mem.contains(&target) {
                return true;
            }
            for val in mem.clone() {
                mem.insert(val + nums[i]);
            }
        }
        false
    }
}
