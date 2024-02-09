pub struct Solution;

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut vals: Vec<Vec<i32>> = Vec::new();
        let mut nums_sorted = nums.clone();
        nums_sorted.sort();
        for i in 0..nums_sorted.len() {
            if i >= 1 && nums_sorted[i] == nums_sorted[i - 1] {
                continue;
            } else {
                let mut lp = i + 1;
                let mut rp = nums.len() - 1;
                while lp < rp {
                    let sum = nums_sorted[i] + nums_sorted[rp] + nums_sorted[lp];
                    if sum > 0 {
                        rp -= 1;
                    } else if sum < 0 {
                        lp += 1;
                    } else {
                        vals.push(vec![nums_sorted[i], nums_sorted[rp], nums_sorted[lp]]);
                        lp += 1;
                        while nums_sorted[lp] == nums_sorted[lp - 1] && lp < rp {
                            lp += 1;
                        }
                    }
                }
            }
        }
        vals
    }
}
