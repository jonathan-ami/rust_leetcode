use std::collections::HashMap;
pub struct Solution {}
impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        if t.len() == 0 || t.len() > s.len() {
            return "".to_string();
        }
        let mut map: HashMap<char, i32> = HashMap::new();
        let mut window: HashMap<char, i32> = HashMap::new();
        let s_chars: Vec<char> = s.chars().collect();
        for c in t.chars() {
            map.entry(c).and_modify(|e| *e += 1).or_insert(1);
        }
        let mut have = 0;
        let need = map.len();
        let mut result: (i32, i32) = (-1, -1);
        let mut length = i32::MAX;
        let mut lp = 0;
        for rp in 0..s.len() {
            let c = s_chars[rp];
            window.entry(c).and_modify(|e| *e += 1).or_insert(1);
            if map.contains_key(&c) {
                let window_count = window.get(&c).unwrap();
                let map_count = map.get(&c).unwrap();
                if *window_count == *map_count {
                    have += 1;
                }
            }
            while have == need {
                if (rp - lp) as i32 + 1 < length {
                    result = (lp as i32, rp as i32);
                    length = (rp - lp + 1) as i32;
                }
                window.entry(s_chars[lp]).and_modify(|e| *e -= 1);
                if map.contains_key(&s_chars[lp]) {
                    let window_count = window.get(&s_chars[lp]).unwrap();
                    let map_count = map.get(&s_chars[lp]).unwrap();
                    if window_count < map_count {
                        have -= 1;
                    }
                }
                lp += 1;
            }
        }
        if length != i32::MAX {
            s[result.0 as usize..result.1 as usize + 1].to_string()
        } else {
            "".to_string()
        }
    }
}
