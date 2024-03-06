use std::{cmp::max, collections::HashSet};

pub struct Solution {}
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut length = 0;
        let mut lp = 0;
        let c: Vec<char> = s.chars().collect();
        let mut set: HashSet<char> = HashSet::new();
        println!("{}", c.len());
        for i in 0..c.len() {
            while set.contains(&c[i]) {
                set.remove(&c[lp]);
                lp += 1;
            }
            length = max(i - lp + 1, length);
            set.insert(c[i]);
        }
        length as i32
    }
}
