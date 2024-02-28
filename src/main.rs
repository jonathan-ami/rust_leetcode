mod search_rotated;
use search_rotated::Solution;
fn main() {
    let piles = vec![4, 5, 6, 7, 0, 1, 2];
    let result = Solution::search(piles, 0);
    print!("{:?}", result);
}
