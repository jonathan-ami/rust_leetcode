struct Solution;
impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        use std::collections::HashSet;
        const ARR_SIZE: usize = 9;
        let mut rows: [HashSet<char>; 9] = Default::default();
        for row_index in 0..board.len() {
            let cur_row = board.get(row_index);
            let mut column: HashSet<char> = HashSet::new();
            for column_index in 0..cur_row.len() {
                let cur_char: char = cur_row.get(column_index);
                if !column.contains(cur_char) {
                    column.insert(cur_char);
                } else {
                    false
                }

                if !rows[column_index].contains(cur_char) {
                    rows[column_index].insert(cur_char);
                } else {
                    false
                }
            }
        }
        true
    }
}
