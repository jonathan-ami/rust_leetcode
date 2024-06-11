pub struct Solution {}
impl Solution {
    pub fn pacific_atlantic(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        use std::collections::HashSet;
        let mut res: Vec<Vec<i32>> = Vec::new();
        for i in 0..heights.len() as i32 {
            for j in 0..heights[0].len() as i32 {
                let mut visited: HashSet<(i32, i32)> = HashSet::new();
                let mut pacific = false;
                let mut atlantic = false;
                dfs(
                    &heights,
                    i,
                    j,
                    i32::MAX,
                    &mut pacific,
                    &mut atlantic,
                    &mut visited,
                );
                if pacific && atlantic {
                    res.push(vec![i, j]);
                }
            }
        }
        fn dfs(
            heights: &Vec<Vec<i32>>,
            x: i32,
            y: i32,
            prev: i32,
            pacific: &mut bool,
            atlantic: &mut bool,
            visited: &mut HashSet<(i32, i32)>,
        ) {
            if visited.contains(&(x, y)) {
                return;
            }
            if x > heights.len() as i32 || y < 0 {
                *atlantic = true;
                return;
            }
            if x < 0 || y > heights[0].len() as i32 {
                *pacific = true;
                return;
            }
            if heights[x as usize][y as usize] > prev {
                return;
            }
            let dirs: [(i32, i32); 4] = [(0, 1), (1, 0), (-1, 0), (0, -1)];
            let prev = heights[x as usize][y as usize];
            visited.insert((x, y));
            for dir in dirs {
                dfs(
                    heights,
                    x + dir.0,
                    y + dir.1,
                    prev,
                    pacific,
                    atlantic,
                    visited,
                );
            }
        }
        res
    }
}
