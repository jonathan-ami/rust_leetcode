pub struct Solution {}
impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return nums[0];
        } else if nums.len() == 0 {
            return 0;
        }
        std::cmp::max(
            Self::r(&nums[1..nums.len()]),
            Self::r(&nums[0..nums.len() - 1]),
        )
    }

    fn r(nums: &[i32]) -> i32 {
        let (mut r1, mut r2) = (0, 0);
        for val in nums {
            let temp = std::cmp::max(r1 + val, r2);
            r1 = r2;
            r2 = temp;
        }
        std::cmp::max(r1, r2)
    }
}
