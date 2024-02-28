pub struct Solution {}
impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let mut lp = 0;
        let mut rp = nums.len() - 1;
        let mut result = nums[0];
        while lp <= rp {
            if nums[lp] < nums[rp] {
                if nums[lp] < result {
                    result = nums[lp];
                }
            }

            let mid = (lp + rp) / 2;
            if nums[mid] < result {
                result = nums[mid];
            }

            if nums[mid] >= nums[lp] {
                lp = mid + 1;
            } else {
                rp = mid - 1;
            }
        }
        result
    }
}
