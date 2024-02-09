mod three_sum;
use three_sum::Solution;

fn main() {
    let values = vec![-1, 0, 1, 2, -1, -4];
    let result = Solution::three_sum(values);
    print!("{:?}", result);
}
