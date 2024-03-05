mod buy_and_sell_stock;
use buy_and_sell_stock::Solution;
fn main() {
    let nums1 = vec![7, 1, 5, 3, 6, 4];
    let result = Solution::max_profit(nums1);
    print!("{:?}", result);
}
