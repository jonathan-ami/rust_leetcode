pub struct Solution {}
impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let rows = matrix.len() - 1;
        let columns = matrix[0].len();

        let mut bottom = rows;
        let mut top = 0;
        while top <= bottom {
            let row = (top + bottom) / 2 as usize;
            if target > matrix[row][columns - 1] {
                top = row + 1;
            } else if target < matrix[row][0] {
                if row > 0 {
                    bottom = row - 1;
                } else {
                    return false;
                }
            } else {
                break;
            }
        }
        if top > bottom {
            return false;
        }
        let mut lp = 0;
        let mut rp = columns - 1;

        let row = (top + bottom) / 2;
        println!("row: {}", row);

        while lp <= rp {
            let m = (lp + rp) / 2;
            if target > matrix[row][m] {
                lp = m + 1;
            } else if target < matrix[row][m] {
                if m > 0 {
                    rp = m - 1;
                }
            } else {
                return true;
            }
        }
        false
    }
}
