use std::cmp::max;

pub struct Solution {}
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut profit = 0;
        let mut lp = 0;
        let mut rp = 1;
        profit = prices[rp] - prices[lp];
        while rp < prices.len() {
            if prices[lp] < prices[rp] {
                profit = max(profit, prices[rp] - prices[lp]);
            } else {
                lp = rp;
            }
            rp += 1;
        }

        profit
    }
}
