pub struct Solution {}

impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut stack: Vec<usize> = Vec::new();
        let mut result: Vec<i32> = vec![0; temperatures.len()];
        stack.push(0);
        for i in 1..temperatures.len() {
            while !stack.is_empty() && temperatures[i] > temperatures[*stack.last().unwrap()] {
                let val = stack.pop().unwrap();
                result[val] = (i - val) as i32;
            }
            stack.push(i);
        }
        result
    }
}
