pub struct Solution {}
impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        use std::collections::HashMap;
        let mut number_combos = HashMap::new();

        number_combos.insert('2', "abc");
        number_combos.insert('3', "def");
        number_combos.insert('4', "ghi");
        number_combos.insert('5', "jkl");
        number_combos.insert('6', "mno");
        number_combos.insert('7', "pqrs");
        number_combos.insert('8', "tuv");
        number_combos.insert('9', "wxyz");

        fn rec(
            i: usize,
            map: &HashMap<char, &str>,
            digits: &Vec<char>,
            set: String,
            res: &mut Vec<String>,
        ) {
            if set.len() == digits.len() {
                res.push(set.clone().to_string());
                return;
            }
            let digit = digits[i];
            for c in map.get(&digit).unwrap().chars() {
                let mut new_set = set.clone();
                new_set.push(c);
                rec(i + 1, map, digits, new_set, res);
            }
        }

        let mut res = Vec::new();
        if digits == "" {
            return Vec::new();
        }
        let digits: Vec<char> = digits.chars().collect();
        rec(0, &number_combos, &digits, String::new(), &mut res);
        res
    }
}
