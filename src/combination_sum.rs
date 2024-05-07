pub struct Solution {}
impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        fn rec(
            i: usize,
            candidates: &Vec<i32>,
            subset: &mut Vec<i32>,
            res: &mut Vec<Vec<i32>>,
            target: i32,
            mut sum: i32,
        ) {
            if i >= candidates.len() || sum > target {
                return;
            }
            if sum == target {
                res.push(subset.clone());
                return;
            }
            subset.push(candidates[i]);
            rec(i, candidates, subset, res, target, sum + candidates[i]);
            let n = subset.pop().unwrap();
            rec(i + 1, candidates, subset, res, target, sum);
        }

        let mut res: Vec<Vec<i32>> = Vec::new();
        let mut subset: Vec<i32> = Vec::new();
        rec(0, &candidates, &mut subset, &mut res, target, 0);
        res
    }
}
