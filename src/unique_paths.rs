pub struct Solution {}

impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let mut mem: Vec<Vec<i32>> = vec![vec![0; n as usize]; m as usize];
        mem[m as usize - 1][n as usize - 1] = 1;
        for i in (0..m).rev() {
            for k in (0..n).rev() {
                if k == n - 1 && i == m - 1 {
                    continue;
                }
                let (d1, d2) = {
                    let (mut d1, mut d2) = (0, 0);
                    if i + 1 < m {
                        d1 = mem[i as usize + 1][k as usize];
                    }
                    if k + 1 < n {
                        d2 = mem[i as usize][k as usize + 1];
                    }
                    (d1, d2)
                };
                mem[i as usize][k as usize] = d1 + d2;
            }
        }
        mem[0][0]
    }
}
