pub struct Solution {}
impl Solution {
    pub fn rain_water(height: Vec<i32>) -> i32 {
        let mut rp: usize = height.len() - 1;
        let mut lp: usize = 0;
        let mut max_left: i32 = height[lp];
        let mut max_right: i32 = height[rp];
        let mut water: i32 = 0;
        while rp > lp {
            if height[rp] > height[lp] {
                let vol = max_left - height[lp];
                if vol > 0 {
                    water += vol;
                }
                lp += 1;
                if height[lp] > max_left {
                    max_left = height[lp];
                }
            } else {
                let vol = max_right - height[rp];
                if vol > 0 {
                    water += vol;
                }
                rp -= 1;
                if height[rp] > max_right {
                    max_right = height[rp];
                }
            }
        }

        water
    }
}
