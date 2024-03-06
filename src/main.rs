mod longest_substring;
use longest_substring::Solution;
fn main() {
    let s = "au".to_string();
    let result = Solution::length_of_longest_substring(s);
    print!("{:?}", result);
}
