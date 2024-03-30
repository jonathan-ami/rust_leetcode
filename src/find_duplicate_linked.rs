pub struct Solution {}
impl Solution {
    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        let mut fp = 0;
        let mut sp = 0;
        loop {
            fp = nums[fp] as usize;
            fp = nums[fp] as usize;
            sp = nums[sp] as usize;
            if fp == sp {
                break;
            }
        }
        let mut sp2 = 0;
        loop {
            sp2 = nums[sp2] as usize;
            sp = nums[sp] as usize;
            if sp == sp2 {
                return sp as i32;
            }
        }
    }
}
