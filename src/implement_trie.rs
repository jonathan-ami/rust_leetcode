use std::collections::HashMap;
pub struct Trie {
    children: HashMap<char, Trie>,
    end: bool,
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
            cur = cur.children.entry(c).or_insert(Trie::new());
        }
        cur.end = true;
    }
    fn starts_with(&self, prefix: String) -> bool {
        let mut cur = self;
        for c in prefix.chars() {
            if cur.children.contains_key(&c) {
                cur = cur.children.get(&c).unwrap();
            } else {
                return false;
            }
        }
        true
    }
    fn search(&self, word: String) -> bool {
        let mut cur = self;
        for c in word.chars() {
            if let Some(node) = cur.children.get(&c) {
                cur = node;
            } else {
                return false;
            }
        }
        cur.end
    }
}
