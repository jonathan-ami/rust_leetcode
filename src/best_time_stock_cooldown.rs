use std::collections::HashMap;
pub struct Solution {}
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut map: HashMap<(i32, bool), i32> = HashMap::new();
        Self::dfs(0, true, &prices, &mut map)
    }

    fn dfs(i: i32, buy: bool, prices: &Vec<i32>, map: &mut HashMap<(i32, bool), i32>) -> i32 {
        if i > prices.len() as i32 {
            return 0;
        }
        if map.contains_key(&(i, buy)) {
            return *map.get(&(i, buy)).unwrap();
        }
        let cooldown = Self::dfs(i + 1, buy, prices, map);
        if buy {
            let val = Self::dfs(i + 1, !buy, prices, map) - prices[i as usize];
            map.insert((i, buy), std::cmp::max(val, cooldown));
        } else {
            let sell = Self::dfs(i + 2, !buy, prices, map) + prices[i as usize];
            map.insert((i, buy), std::cmp::max(sell, cooldown));
        }
        *map.get(&(i, buy)).unwrap()
    }
}
