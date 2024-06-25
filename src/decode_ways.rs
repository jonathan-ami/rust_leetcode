use std::collections::HashMap;
pub struct Solution {}
impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        let s: Vec<char> = s.chars().collect();
        let (mut d1, mut d2, mut d3) = (0, 1, 0);
        for i in (0..s.len() as i32 - 1).rev() {
            if s[i as usize] != '0' {
                d1 += d2;
            }
            if i as usize + 1 < s.len()
                && (s[i as usize] == '1'
                    || s[i as usize] == '2' && s[i as usize + 1].to_digit(10).unwrap() <= 6)
            {
                d2 += d3;
            }
            d3 = d2;
            d2 = d1;
            d1 = 0;
        }
        d2
    }
}
