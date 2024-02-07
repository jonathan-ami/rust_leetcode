use std::collections::HashSet;
pub struct Solution;
impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let hash_set: HashSet<i32> = nums.into_iter().collect();
        let mut count: i32 = 0;
        for val in &hash_set {
            let mut con: i32 = 0;
            if !hash_set.contains(&(val - 1)) {
                while hash_set.contains(&(val + con)) {
                    con += 1;
                }
            }
            if con > count {
                count = con;
            }
        }
        count
    }
}
