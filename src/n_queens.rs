pub struct Solution {}
impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        use std::collections::HashSet;
        let mut cols = HashSet::new();
        let mut neg = HashSet::new();
        let mut pos = HashSet::new();
        let mut res: Vec<Vec<String>> = Vec::new();
        let mut sub: Vec<String> = Vec::new();
        for i in 0..n as usize {
            sub[i] = String::from("....");
        }
        fn rec(
            cols: &mut HashSet<i32>,
            neg: &mut HashSet<i32>,
            pos: &mut HashSet<i32>,
            row: i32,
            n: i32,
            sub: &mut Vec<String>,
            res: &mut Vec<Vec<String>>,
        ) {
            if row > n {
                res.push(sub.clone());
                return;
            }
            for i in 0..n {
                if cols.contains(&i) || neg.contains(&(row - i)) || pos.contains(&(i + row)) {
                    continue;
                }

                let mut chars: Vec<char> = sub[row as usize].chars().collect();
                chars[i as usize] = 'Q';
                cols.insert(i);
                neg.insert(row - i);
                pos.insert(i - row);
                sub[row as usize] = chars.clone().into_iter().collect();
                rec(cols, neg, pos, row + 1, n, sub, res);
                cols.remove(&i);
                neg.remove(&(row - i));
                pos.remove(&(i - row));
                chars[i as usize] = '.';
                sub[row as usize] = chars.clone().into_iter().collect();
            }
        }
        rec(&mut cols, &mut neg, &mut pos, 0, n, &mut sub, &mut res);
        res
    }
}
