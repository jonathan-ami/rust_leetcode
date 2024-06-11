pub struct Solution {}
impl Solution {
    pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        use std::collections::HashMap;
        use std::collections::HashSet;
        use std::collections::VecDeque;

        if word_list.contains(&end_word) {
            return 0;
        }
        let mut map: HashMap<String, Vec<String>> = HashMap::new();
        for word in word_list {
            for i in 0..word.len() {
                let mut w: Vec<char> = word.clone().chars().collect();
                w[i] = '*';
                let w2: String = w.clone().into_iter().collect();
                map.entry(word.clone())
                    .and_modify(|e| e.push(w2.clone()))
                    .or_insert(vec![w2.clone()]);
            }
        }
        let mut visited: HashSet<String> = HashSet::new();
        let mut q: VecDeque<String> = VecDeque::new();
        let mut res = 1;
        while !q.is_empty() {
            for i in 0..q.len() {
                let word = q.pop_back().unwrap();
                if word == end_word {
                    return res;
                }
                for j in 0..word.len() {
                    let mut w: Vec<char> = word.chars().collect();
                    w[j] = '*';
                    let w: String = w.into_iter().collect();
                    let vals = map.get(&w).unwrap();
                    for w2 in vals {
                        if !visited.contains(w2) {
                            visited.insert(w2.clone());
                            q.push_front(w2.clone())
                        }
                    }
                }
            }
            res += 1;
        }
        return 0;
    }
}
