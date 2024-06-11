pub struct Solution {}
impl Solution {
    pub fn oranges_rotting(mut grid: Vec<Vec<i32>>) -> i32 {
        use std::collections::VecDeque;
        let mut queue = VecDeque::new();
        let mut res = 0;
        let mut fresh = 0;
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == 2 {
                    queue.push_back((i as i32, j as i32));
                } else if grid[i][j] == 1 {
                    fresh += 1;
                }
            }
        }
        let dirs: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
        while !queue.is_empty() && fresh > 0 {
            for i in 0..queue.len() {
                let (r, c) = queue.pop_front().unwrap();
                for dir in dirs {
                    let (row, col) = (dir.0 + r, dir.1 + c);
                    if row < 0
                        || row as usize >= grid.len()
                        || col < 0
                        || col as usize >= grid[0].len()
                        || grid[row as usize][col as usize] != 1
                    {
                        continue;
                    }
                    grid[row as usize][col as usize] = 2;
                    queue.push_back((row, col));
                    fresh -= 1;
                }
            }
            res += 1;
        }
        if fresh == 0 {
            return res;
        }
        -1
    }
}
