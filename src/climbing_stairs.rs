pub struct Solution {}
use std::collections::HashMap;
impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let (mut one, mut two) = (1, 1);
        for i in 0..n {
            let temp = two;
            two = one + two;
            one = temp;
        }
        two
    }
}
