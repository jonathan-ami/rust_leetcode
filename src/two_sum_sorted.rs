pub struct Solution;

impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut rp: usize = numbers.len() - 1;
        let mut lp: usize = 0;
        while rp > lp {
            if numbers[lp] + numbers[rp] > target {
                rp -= 1;
            } else if numbers[lp] + numbers[rp] > target {
                lp += 1;
            } else {
                let result: Vec<i32> = vec![(lp + 1) as i32, (rp + 1) as i32];
                return result;
            }
        }
        return vec![0];
    }
}
