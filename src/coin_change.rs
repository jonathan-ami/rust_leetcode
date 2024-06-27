pub struct Solution {}
impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let mut mem = vec![amount + 1; amount as usize + 1];
        mem[0] = 0;

        for i in 1..amount + 1 {
            for c in &coins {
                if i - c >= 0 {
                    mem[i as usize] = std::cmp::min(mem[i as usize], 1 + mem[(i - c) as usize]);
                }
            }
        }
        if mem[amount as usize] != amount + 1 {
            return mem[amount as usize];
        }
        -1
    }
}
