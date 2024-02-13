mod daily_temperatures;
use daily_temperatures::Solution;

fn main() {
    let values = vec![73, 74, 75, 71, 69, 72, 76, 73];
    let result = Solution::daily_temperatures(values);
    print!("{:?}", result);
}
