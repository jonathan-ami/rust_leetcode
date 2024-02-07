mod generate_parentheses;
use generate_parentheses::Solution;

fn main() {
    let result = Solution::gen_parentheses(3);
    print!("{:?}", result);
}
