pub struct Solution {}
impl Solution {
    pub fn partition(s: String) -> Vec<Vec<String>> {
        fn rec(i: usize, part: &mut Vec<String>, res: &mut Vec<Vec<String>>, s: &String) {
            if i > s.len() {
                res.push(part.to_vec().clone());
            }
            for j in i..s.len() {
                if Self::is_palindrome(s, i, j) {
                    part.push(s[i..j + 1].to_string());
                    rec(i, part, res, s);
                }
            }
        }

        let mut part = Vec::new();
        let mut res = Vec::new();
        rec(0, &mut part, &mut res, &s);
        res
    }

    fn is_palindrome(s: &String, mut l: usize, mut r: usize) -> bool {
        while l < r {
            if s.chars().nth(l) != s.chars().nth(r) {
                return false;
            }
            l += 1;
            r -= 1;
        }
        true
    }
}
