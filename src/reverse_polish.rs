pub struct Solution {}

impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack: Vec<i32> = Vec::new();
        let mut output: i32 = 0;
        for token in tokens {
            if let Ok(num) = token.parse::<i32>() {
                stack.push(num);
            } else {
                let num2 = stack.pop();
                let num1 = stack.pop();
                let mut result: i32 = 0;

                if let (Some(num1), Some(num2)) = (num1, num2) {
                    if token == "+" {
                        result = num1 + num2;
                    } else if token == "-" {
                        result = num1 - num2;
                    } else if token == "/" {
                        result = num1 / num2;
                    } else if token == "*" {
                        result = num1 * num2;
                    }
                    stack.push(result);
                }
            }
        }
        stack.pop().unwrap()
    }
}
