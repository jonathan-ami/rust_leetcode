use std::collections::HashSet;

pub struct Solution {}
impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        fn rec(
            i: usize,
            x: i32,
            y: i32,
            board: &Vec<Vec<char>>,
            word: &String,
            path: &mut HashSet<(i32, i32)>,
        ) -> bool {
            if i == word.len() {
                return true;
            }
            if x < 0
                || y < 0
                || x as usize >= board.len()
                || y as usize >= board[0].len()
                || path.contains(&(x, y))
                || word.chars().nth(i).unwrap() != board[x as usize][y as usize]
            {
                return false;
            }
            path.insert((x, y));
            let res = rec(i + 1, x + 1, y, board, word, path)
                || rec(i + 1, x, y + 1, board, word, path)
                || rec(i + 1, x - 1, y, board, word, path)
                || rec(i + 1, x, y - 1, board, word, path);
            path.remove(&(x, y));
            res
        }
        let mut path: HashSet<(i32, i32)> = HashSet::new();
        for x in 0..board.len() {
            for y in 0..board[0].len() {
                if rec(0, x as i32, y as i32, &board, &word, &mut path) {
                    return true;
                }
            }
        }
        false
    }
}
