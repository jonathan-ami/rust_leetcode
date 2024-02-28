pub struct Solution {}
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut lp = 0;
        let mut rp = nums.len() - 1;

        while lp <= rp {
            let mid = (lp + rp) / 2;
            if nums[mid] == target {
                return mid as i32;
            }

            if nums[lp] <= nums[mid] {
                if nums[mid] < target || nums[lp] > target {
                    lp = mid + 1;
                } else if nums[mid] > target || nums[rp] < target {
                    rp = mid - 1;
                }
            } else {
                if target < nums[mid] || target > nums[lp] {
                    rp = mid - 1;
                } else {
                    lp = mid + 1;
                }
            }
        }

        -1
    }
}
