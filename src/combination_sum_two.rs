pub struct Solution {}
impl Solution {
    pub fn combination_sum2(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        fn rec(
            mut i: usize,
            candidates: &Vec<i32>,
            subset: &mut Vec<i32>,
            res: &mut Vec<Vec<i32>>,
            target: i32,
            sum: i32,
        ) {
            if i >= candidates.len() || sum > target {
                return;
            }
            if sum == target {
                res.push(subset.to_vec().clone());
                return;
            }
            subset.push(candidates[i]);
            rec(i + 1, candidates, subset, res, target, sum + candidates[i]);
            subset.pop();
            while i + 1 < candidates.len() && candidates[i] == candidates[i + 1] {
                i = i + 1;
            }
            rec(i + 1, candidates, subset, res, target, sum);
        }
        candidates.sort();
        let mut res: Vec<Vec<i32>> = Vec::new();
        let mut subset: Vec<i32> = Vec::new();
        rec(0, &candidates, &mut subset, &mut res, target, 0);
        res
    }
}
