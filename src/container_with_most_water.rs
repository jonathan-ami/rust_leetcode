pub struct Solution {}

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut rp: usize = height.len() - 1;
        let mut lp: usize = 0;
        let mut max: i32 = 0;
        while rp > lp {
            let area = (rp - lp) as i32 * std::cmp::min(height[lp], height[rp]);
            if area > max {
                max = area;
            }
            if height[lp] < height[rp] {
                lp += 1;
            } else {
                rp -= 1;
            }
        }
        max
    }
}
