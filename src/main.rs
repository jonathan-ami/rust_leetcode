mod car_fleet;
use car_fleet::Solution;

fn main() {
    let position = vec![10, 8, 0, 5, 3];
    let speed = vec![2, 4, 1, 1, 3];
    let result = Solution::car_fleet(12, position, speed);
    print!("{:?}", result);
}
