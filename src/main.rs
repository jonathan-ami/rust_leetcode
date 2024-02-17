mod trapping_rain_water;
use trapping_rain_water::Solution;
fn main() {
    let heights = vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1];
    let result = Solution::rain_water(heights);
    print!("{:?}", result);
}
