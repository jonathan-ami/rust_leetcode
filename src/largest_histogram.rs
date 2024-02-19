pub struct Solution {}

impl Solution {
    pub fn largest_area(heights: Vec<i32>) -> i32 {
        let mut largest_area: i32 = 0;
        let mut stack: Vec<(usize, i32)> = Vec::new();
        for i in 0..heights.len() {
            let mut start = i;
            while !stack.is_empty() && heights[i] < stack.last().unwrap().1 {
                let val = stack.pop().unwrap();
                let area = val.1 * (i - val.0) as i32;
                if area > largest_area {
                    largest_area = area;
                }
                start = val.0;
            }
            stack.push((start, heights[i]));
        }

        for val in stack {
            let area = (heights.len() - val.0) as i32 * val.1;
            if area > largest_area {
                largest_area = area;
            }
        }

        largest_area
    }
}
