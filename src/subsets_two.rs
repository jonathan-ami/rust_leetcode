pub struct Solution {}
impl Solution {
    pub fn subsets_with_dup(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        fn rec(mut i: usize, res: &mut Vec<Vec<i32>>, subset: &mut Vec<i32>, nums: &Vec<i32>) {
            if i >= nums.len() {
                res.push(subset.clone());
                return;
            }
            subset.push(nums[i]);
            rec(i + 1, res, subset, nums);
            subset.pop();
            while i + 1 < nums.len() && nums[i] == nums[i + 1] {
                i += 1;
            }
            rec(i + 1, res, subset, nums);
        }

        let mut res = Vec::new();
        let mut subset = Vec::new();
        nums.sort();
        rec(0, &mut res, &mut subset, &nums);
        res
    }
}
