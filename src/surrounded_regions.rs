pub struct Solution {}
impl Solution {
    pub fn solve(board: &mut Vec<Vec<char>>) {
        use std::collections::HashSet;
        let mut set: HashSet<(i32, i32)> = HashSet::new();
        for i in 0..board.len() as i32 {
            for j in 0..board[0].len() as i32 {
                if board[i as usize][j as usize] == 'O'
                    && ((i == 0 || i == board.len() as i32 - 1)
                        || (j == 0 || j == board[0].len() as i32 - 1))
                {
                    dfs(board, &mut set, i, j);
                }
            }
        }
        for i in 0..board.len() {
            for j in 0..board[0].len() {
                if board[i][j] == 'O' && !set.contains(&(i as i32, j as i32)) {
                    board[i][j] = 'X';
                }
            }
        }

        fn dfs(board: &mut Vec<Vec<char>>, set: &mut HashSet<(i32, i32)>, r: i32, c: i32) {
            if r < 0
                || r >= board.len() as i32
                || c < 0
                || c >= board[0].len() as i32
                || board[r as usize][c as usize] == 'X'
                || set.contains(&(r, c))
            {
                return;
            }
            set.insert((r, c));
            let dirs: [(i32, i32); 4] = [(0, 1), (1, 0), (-1, 0), (0, -1)];
            for dir in dirs {
                let (row, col) = (r + dir.0, c + dir.1);
                dfs(board, set, row, col);
            }
        }
    }
}
