use std::collections::HashMap;
pub struct Solution {}
impl Solution {
    pub fn change(amount: i32, coins: Vec<i32>) -> i32 {
        let mut mem: HashMap<(usize, i32), i32> = HashMap::new();
        Self::dfs(0, 0, amount, &coins, &mut mem)
    }

    fn dfs(
        sum: i32,
        i: usize,
        amount: i32,
        coins: &Vec<i32>,
        mem: &mut HashMap<(usize, i32), i32>,
    ) -> i32 {
        if sum > amount || i >= coins.len() {
            return 0;
        }

        if sum == amount {
            return 1;
        }

        if let Some(val) = mem.get(&(i, sum)) {
            return *val;
        }
        let val = Self::dfs(sum + coins[i], i, amount, coins, mem)
            + Self::dfs(sum, i + 1, amount, coins, mem);
        mem.insert((i, sum), val);
        val
    }
}
