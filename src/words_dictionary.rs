use std::collections::HashMap;

struct WordDictionary {
    children: HashMap<char, WordDictionary>,
    end: bool,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl WordDictionary {
    fn new() -> Self {
        WordDictionary {
            children: HashMap::new(),
            end: false,
        }
    }

    fn add_word(&mut self, word: String) {
        let mut cur = self;
        for c in word.chars() {
            cur = cur.children.entry(c).or_insert(WordDictionary::new());
        }
        cur.end = true;
    }

    fn search(&self, word: String) -> bool {
        fn dfs(root: &WordDictionary, word: String) -> bool {
            let mut cur = root;
            let mut i = 0;
            for c in word.chars() {
                if c == '.' {
                    for node in cur.children.values() {
                        let word = &word[i + 1..];
                        if dfs(node, word.to_string()) {
                            return true;
                        }
                    }
                    return false;
                }
                if let Some(val) = cur.children.get(&c) {
                    cur = val;
                } else {
                    return false;
                }
                i += 1;
            }
            cur.end
        }
        dfs(self, word)
    }
}

