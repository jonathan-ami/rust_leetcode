pub struct Solution {}
impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        fn rec(i: usize, nums: Vec<i32>, res: &mut Vec<Vec<i32>>, sub: &mut Vec<i32>) {
            if i >= nums.len() {
                res.push(sub.clone());
                return;
            }
            sub.push(nums[i]);
            rec(i + 1, nums.clone(), res, sub);
            sub.pop();
            rec(i + 1, nums.clone(), res, sub);
        }
        let mut res: Vec<Vec<i32>> = Vec::new();
        let mut sub: Vec<i32> = Vec::new();
        rec(0, nums, &mut res, &mut sub);
        res
    }
}
