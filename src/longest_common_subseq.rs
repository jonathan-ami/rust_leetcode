pub struct Solution {}
impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let mut dp = vec![vec![0; text2.len() + 1]; text1.len() + 1];
        let t1 = text1.as_bytes();
        let t2 = text2.as_bytes();
        for i in (0..text1.len()).rev() {
            for j in (0..text2.len()).rev() {
                if t1[i] == t2[j] {
                    dp[i][j] = 1 + dp[i + 1][j + 1];
                } else {
                    dp[i][j] = std::cmp::max(dp[i][j + 1], dp[i + 1][j]);
                }
            }
        }
        dp[0][0]
    }
}
