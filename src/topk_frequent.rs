use std::collections::HashMap;

pub struct Solution;
impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut map: HashMap<i32, i32> = HashMap::new();

        for element in nums {
            map.entry(element).and_modify(|x| *x += 1).or_insert(1);
        }
        let mut result: Vec<(i32, i32)> = map.into_iter().collect();
        result.sort_by(|a, b| b.1.cmp(&a.1));
        for elemeent in &result {
            print!("{:?}", elemeent);
        }
        result
            .iter()
            .take(k as usize)
            .map(|(value, _)| *value)
            .collect()
    }
}
