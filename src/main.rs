mod eating_bananas;
use eating_bananas::Solution;
fn main() {
    let piles = vec![312884470];
    let result = Solution::min_eating_speed(piles, 312884469);
    print!("{:?}", result);
}
