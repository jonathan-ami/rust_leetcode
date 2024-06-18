use std::ops::RangeInclusive;
pub struct Solution {}
impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let s = s.as_str();
        (0..s.len())
            .fold("", |current_longest, idx| {
                current_longest
                    .longest(s.longest_palindrome_around(idx..=idx))
                    .longest(s.longest_palindrome_around(idx..=idx + 1))
            })
            .into()
    }
}

trait LongestPalindrome {
    type Idx;
    fn longest_palindrome_around(&self, center: RangeInclusive<Self::Idx>) -> &Self;
    fn longest<'a>(&'a self, other: &'a Self) -> &'a Self;
}
impl LongestPalindrome for str {
    type Idx = usize;
    fn longest_palindrome_around(&self, center: RangeInclusive<Self::Idx>) -> &Self {
        let (mut start, mut end) = center.into_inner();
        let characters = self.as_bytes();
        loop {
            if characters.get(start) != characters.get(end) {
                return &self[start + 1..end];
            }
            if let (Some(new_start), Some(new_end)) = (start.checked_sub(1), end.checked_add(1)) {
                start = new_start;
                end = new_end;
            } else {
                return &self[start..=end];
            }
        }
    }
    fn longest<'a>(&'a self, other: &'a Self) -> &'a Self {
        if self.len() > other.len() {
            self
        } else {
            other
        }
    }
}
