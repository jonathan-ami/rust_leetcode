pub struct Solution {}
impl Solution {
    pub fn max_area_of_island(grid: Vec<Vec<i32>>) -> i32 {
        use std::collections::HashSet;
        use std::collections::VecDeque;
        fn bfs(visited: &mut HashSet<(i32, i32)>, row: i32, col: i32, grid: &Vec<Vec<i32>>) -> i32 {
            visited.insert((row, col));
            let mut queue = VecDeque::new();
            let mut total = 0;
            queue.push_front((row, col));
            while !queue.is_empty() {
                total += 1;
                let (r, c) = queue.pop_back().unwrap();
                let dirs = [(1, 0), (-1, 0), (0, 1), (0, -1)];
                for dir in dirs {
                    let (row, col) = (r + dir.0, c + dir.1);
                    if row >= 0
                        && col >= 0
                        && row < grid.len() as i32
                        && col < grid.len() as i32
                        && !visited.contains(&(row, col))
                        && grid[row as usize][col as usize] != 0
                    {
                        queue.push_front((row, col));
                    }
                }
            }
            total
        }
        let mut visited: HashSet<(i32, i32)> = HashSet::new();
        let mut max = 0;
        for row in 0..grid.len() as i32 {
            for col in 0..grid[0].len() as i32 {
                if !visited.contains(&(row, col)) {
                    max = std::cmp::max(bfs(&mut visited, row, col, &grid), max);
                }
            }
        }
        max
    }
}
