pub struct Solution {}
impl Solution {
    pub fn min_cost_climbing_stairs(mut cost: Vec<i32>) -> i32 {
        cost.push(0);
        for i in (0..cost.len() as i32 - 2).rev() {
            cost[i as usize] += std::cmp::min(cost[i as usize + 1], cost[i as usize + 2]);
        }
        std::cmp::min(cost[0], cost[1])
    }
}
