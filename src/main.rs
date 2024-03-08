mod longest_repeating_character;
use longest_repeating_character::Solution;
fn main() {
    let s = "AABABBBBB".to_string();
    let result = Solution::character_replacement(s, 0);
    print!("{:?}", result);
}
