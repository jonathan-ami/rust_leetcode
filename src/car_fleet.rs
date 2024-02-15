use std::cmp::Reverse;

pub struct Solution {}

impl Solution {
    pub fn car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
        let mut pos_speed: Vec<(i32, i32)> = position.into_iter().zip(speed.into_iter()).collect();
        let mut stack: Vec<f64> = Vec::new();
        pos_speed.sort_by_key(|&(a, _)| Reverse(a));
        for val in pos_speed {
            let time = (target - val.0) as f64 / val.1 as f64;
            if stack.is_empty() {
                stack.push(time);
            } else {
                let top = stack.last().unwrap();
                if &time > top {
                    stack.push(time);
                }
            }
        }
        stack.len() as i32
    }
}
