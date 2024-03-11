use std::collections::HashMap;
pub struct Solution {}
impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        let mut result: bool = true;
        let mut s1_map: HashMap<char, i32> = HashMap::new();
        let mut s2_map: HashMap<char, i32> = HashMap::new();
        let s1_chars: Vec<char> = s1.chars().collect();
        let s2_chars: Vec<char> = s2.chars().collect();
        let alphabet = vec![
            'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q',
            'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
        ];
        for i in 0..s1.len() {
            s1_map
                .entry(s1_chars[i])
                .and_modify(|e| *e += 1)
                .or_insert(1);
            s2_map
                .entry(s2_chars[i])
                .and_modify(|e| *e += 1)
                .or_insert(1);
        }
        let mut matches = 0;
        for val in alphabet {
            if s1_map.contains_key(&val) && s2_map.contains_key(&val) {
                matches += 1;
            }
        }
        let mut lp = 0;
        for rp in s1_chars.len()..s2_chars.len() {
            if matches == 26 {
                return true;
            }
            if s1_map.entry(rp as i32) == s2_map.entry(rp as i32) {
                matches += 1;
            } else if s1_map.entry(rp as i32) + 1 == s2_map.entry(rp as i32) {
                matches -= 1;
            }
        }

        result
    }
}
