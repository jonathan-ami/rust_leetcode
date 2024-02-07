pub struct Solution;
impl Solution {
    pub fn gen_parentheses(num: i32) -> Vec<String> {
        let val = String::from("");
        let mut return_string: Vec<String> = Vec::new();
        Self::rec_helper(0, 0, &mut (return_string), val, num);
        return_string
    }

    fn rec_helper(
        num_open: i32,
        num_closed: i32,
        strings: &mut Vec<String>,
        val: String,
        num: i32,
    ) {
        if num_open > num || num_closed > num {
            return;
        }
        if num_closed == num && num_open == num {
            strings.push(val.to_string());
        } else {
            let mut string = val.clone();
            let mut string_copy = val.clone();
            if num_open > num_closed {
                string.push_str(")");
                string_copy.push_str("(");
                Self::rec_helper(num_open, num_closed + 1, strings, string, num);
                Self::rec_helper(num_open + 1, num_closed, strings, string_copy, num);
            } else {
                string.push_str("(");
                Self::rec_helper(num_open + 1, num_closed, strings, string, num);
            }
        }
    }
}
