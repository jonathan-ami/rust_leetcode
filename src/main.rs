mod sliding_window_max;
use sliding_window_max::Solution;
fn main() {
    let nums = vec![1, 3, -1, -3, 5, 3, 6, 7];
    let result = Solution::max_sliding_window(nums, 3);
    print!("{:?}", result);
}
