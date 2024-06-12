pub struct Solution {}
impl Solution {
    pub fn robs(nums: Vec<i32>) -> i32 {
        let (mut r1, mut r2) = (0, 0);
        for num in nums {
            let temp = std::cmp::max(num + r1, r2);
            r1 = temp;
            r2 = num;
        }
        std::cmp::max(r1, r2)
    }
}
