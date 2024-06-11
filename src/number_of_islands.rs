pub struct Solution {}
impl Solution {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        use std::collections::HashSet;
        use std::collections::VecDeque;
        if grid.is_empty() {
            return 0;
        }
        let mut set: HashSet<(i32, i32)> = HashSet::new();
        let mut res = 0;
        fn bfs(set: &mut HashSet<(i32, i32)>, row: i32, col: i32, grid: &Vec<Vec<char>>) {
            let mut queue = VecDeque::new();
            set.insert((row, col));
            queue.push_back((row, col));
            while !queue.is_empty() {
                let (row, col) = queue.pop_front().unwrap();
                let directions = [(1, 0), (-1, 0), (0, 1), (0, -1)];
                for dir in directions {
                    let (r, c) = (row + dir.0, col + dir.1);
                    if r > 0
                        && r < grid.len() as i32
                        && c > 0
                        && c < grid.len() as i32
                        && !set.contains(&(r, c))
                        && grid[r as usize][c as usize] != '0'
                    {
                        queue.push_back((row + dir.0, col + dir.1));
                        set.insert((row + dir.0, col + dir.1));
                    }
                }
            }
        }
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == '1' && !set.contains(&(i as i32, j as i32)) {
                    bfs(&mut set, i as i32, j as i32, &grid);
                    res += 1;
                }
            }
        }
        res
    }
}

