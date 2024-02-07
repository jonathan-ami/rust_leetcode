pub struct Solution;
impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let only_chars: Vec<char> = s
            .to_lowercase()
            .chars()
            .filter(|c| c.is_alphabetic() || c.is_numeric())
            .collect();
        if only_chars.is_empty() {
            return true;
        }
        let mut p1: usize = 0;
        let mut p2: usize = only_chars.len() - 1;
        while p2 >= only_chars.len() / 2 - 1 {
            if only_chars[p1] != only_chars[p2] {
                return false;
            }
            p1 += 1;
            p2 -= 1;
        }
        true
    }
}
