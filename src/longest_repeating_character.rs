use std::{cmp::max, collections::HashMap};
pub struct Solution {}
impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        let mut result: i32 = 0;
        let mut lp: i32 = 0;
        let mut map: HashMap<char, i32> = HashMap::new();
        let chars: Vec<char> = s.chars().collect();
        let mut max_f = 0;
        for rp in 0..chars.len() {
            let val = map
                .entry(chars[rp as usize])
                .and_modify(|e| *e += 1)
                .or_insert(1);
            max_f = max(val.clone(), max_f);

            while (rp as i32 - lp + 1) - max_f > k {
                map.entry(chars[lp as usize]).and_modify(|e| *e -= 1);
                lp += 1;
            }
            result = max(result, rp as i32 - lp + 1);
        }

        result
    }
}
