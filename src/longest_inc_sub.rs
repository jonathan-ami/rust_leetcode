pub struct Solution {}
impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut mem = vec![0; nums.len()];
        mem[nums.len() - 1] = 1;
        let mut res = 0;
        for i in (0..nums.len() - 1).rev() {
            mem[i] = mem
                .iter()
                .enumerate()
                .filter(|&(x, _)| nums[x] > nums[i] && x > i)
                .map(|(_, &x)| x + 1)
                .max()
                .unwrap_or(1)
                .clone();

            res = std::cmp::max(res, mem[i]);
        }
        std::cmp::max(res, 1)
    }
}
