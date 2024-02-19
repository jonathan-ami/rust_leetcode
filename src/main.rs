mod largest_histogram;
use largest_histogram::Solution;
fn main() {
    let heights = vec![2, 1, 5, 6, 2, 3];
    let result = Solution::largest_area(heights);
    print!("{:?}", result);
}
