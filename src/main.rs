mod minimum_window_substring;
use minimum_window_substring::Solution;
fn main() {
    let s1 = "aa".to_string();
    let s2 = "aa".to_string();
    let result = Solution::min_window(s1, s2);
    print!("{:?}", result);
}
