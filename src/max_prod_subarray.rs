pub struct Solution {}
impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let mut res = nums.iter().max().unwrap().clone() as i64;
        let (mut min, mut max): (i64, i64) = (1, 1);
        for num in nums {
            let num = num as i64;
            let temp = num * max;
            max = std::cmp::max(temp, num * min);
            max = std::cmp::max(num, max);
            min = std::cmp::min(temp, num * min);
            min = std::cmp::min(num, min);
            if min < i32::MIN as i64 {
                min = i32::MIN as i64;
            }
            if max > i32::MAX as i64 {
                max = i32::MAX as i64;
            }
            res = std::cmp::max(max, res);
        }
        res as i32
    }
}
