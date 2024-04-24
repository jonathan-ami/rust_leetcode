pub struct Solution {}
use std::collections::{HashMap, HashSet};
pub struct Trie {
    end: bool,
    children: HashMap<char, Trie>,
}
impl Trie {
    fn new() -> Self {
        Trie {
            end: false,
            children: HashMap::new(),
        }
    }
    fn insert(&mut self, word: String) {
        let mut cur = self;
        for c in word.chars() {
            let cur = cur.children.entry(c).or_insert(Trie::new());
        }
        cur.end = true;
    }
}
impl Solution {
    pub fn find_words(board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
        let mut root = Trie::new();
        for w in words {
            root.insert(w);
        }
        let mut visted: HashSet<(i32, i32)> = HashSet::new();
        let mut res: Vec<String> = Vec::new();
        fn dfs(
            row: i32,
            col: i32,
            node: &mut Trie,
            word: &str,
            board: &Vec<Vec<char>>,
            res: &mut Vec<String>,
            visited: &mut HashSet<(i32, i32)>,
        ) {
            if row < 0
                || col < 0
                || row as usize == board.len()
                || col as usize == board[0].len()
                || visited.contains(&(row, col))
                || !node
                    .children
                    .contains_key(&board[row as usize][col as usize])
            {
                return;
            }
            visited.insert((row, col));

            let mut cur = node
                .children
                .get_mut(&board[row as usize][col as usize])
                .unwrap();
            let mut new_word = word.to_string();
            new_word.push(board[row as usize][col as usize]);
            println!("{:?}", word);
            if cur.end {
                res.push(new_word.clone());
            }
            dfs(row + 1, col, &mut cur, &new_word, board, res, visited);
            dfs(row + 1, col, &mut cur, &new_word, board, res, visited);
            dfs(row, col + 1, &mut cur, &new_word, board, res, visited);
            dfs(row, col - 1, &mut cur, &new_word, board, res, visited);
            visited.remove(&(row, col));
        }
        let mut word = String::new();
        for r in 0..board.len() {
            for c in 0..board[0].len() {
                let mut word: Vec<char> = Vec::new();
                dfs(
                    r as i32,
                    c as i32,
                    &mut root,
                    &"",
                    &board,
                    &mut res,
                    &mut visted,
                );
            }
        }
        res
    }
}
