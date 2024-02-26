pub struct Solution {}
impl Solution {
    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        let max = piles.iter().max().unwrap();
        Self::binary_search(1, *max, h, *max, &piles)
    }
    fn binary_search(start: i32, end: i32, h: i32, k: i32, piles: &Vec<i32>) -> i32 {
        use std::cmp::min;
        let mut k = k;
        if start > end {
            return k;
        }
        let mid = (start + end) / 2;

        let mut time = 0;
        for val in piles {
            let val = (*val as f64 / mid as f64).ceil() as i64;
            time += val;
        }

        if time > h as i64 {
            Self::binary_search(mid + 1, end, h, k, piles)
        } else {
            k = min(k, mid);
            Self::binary_search(start, mid - 1, h, k, piles)
        }
    }
}
