pub struct Solution {}
impl Solution {
    pub fn count_substrings(s: String) -> i32 {
        let s = s.as_bytes();
        let mut res = 0;
        for i in 0..s.len() {
            let (l, mut r) = (i as i32, i as i32);
            res += Self::count(s, l, r);
            r += 1;
            res += Self::count(s, l, r);
        }
        res
    }
    fn count(s: &[u8], mut l: i32, mut r: i32) -> i32 {
        let mut res = 0;
        while l >= 0 && r < s.len() as i32 && s[l as usize] == s[r as usize] {
            res += 1;
            l -= 1;
            r += 1;
        }
        res
    }
}
