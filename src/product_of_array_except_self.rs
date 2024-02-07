pub struct Solution;
impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut product: i32 = 1;
        for num in &nums {
            product *= num;
        }
        let mut return_val: Vec<i32> = Vec::new();
        for num in nums {
            return_val.push(&product / num);
        }
        return_val
    }
}
