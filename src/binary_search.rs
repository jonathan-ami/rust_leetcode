pub struct Solution {}
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        Self::rec_helper(0, (nums.len() - 1) as i32, &nums, target)
    }

    fn rec_helper(start: i32, end: i32, nums: &Vec<i32>, target: i32) -> i32 {
        if start > end {
            return -1;
        }
        let i = start + (end - start) / 2;
        if nums[i as usize] == target {
            return i as i32;
        }

        if nums[i as usize] < target {
            return Self::rec_helper(i + 1, end, nums, target);
        } else {
            return Self::rec_helper(start, i - 1, nums, target);
        }
    }
}
