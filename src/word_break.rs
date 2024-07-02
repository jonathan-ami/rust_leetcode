pub struct Solution {}
impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let mut map = vec![false; word_dict.len() + 1];
        map[word_dict.len() as usize] = true;
        for i in (0..word_dict.len()).rev() {
            for word in &word_dict {
                if i + word.len() <= s.len() && &s[i..i + word.len()] == word {
                    map[i] = map[i + word.len()];
                }
                if map[i] {
                    break;
                }
            }
        }
        return map[0];
    }
}
