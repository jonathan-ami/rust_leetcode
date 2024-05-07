use std::cmp::Reverse;
use std::collections::BinaryHeap;
struct MedianFinder {
    min: BinaryHeap<Reverse<i32>>,
    max: BinaryHeap<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MedianFinder {
    fn new() -> Self {
        MedianFinder {
            min: BinaryHeap::new(),
            max: BinaryHeap::new(),
        }
    }

    fn add_num(&mut self, num: i32) {
        self.max.push(num);
        if self.min.peek().is_some() && self.max.peek().is_some() {
            if self.min.peek().unwrap().0 > *self.max.peek().unwrap() {
                self.min.push(Reverse(self.max.pop().unwrap()));
            }
        }
        if self.max.len() > 1 + self.min.len() {
            self.min.push(Reverse(self.max.pop().unwrap()));
        } else if self.min.len() > 1 + self.max.len() {
            self.max.push(self.min.pop().unwrap().0);
        }
    }

    fn find_median(&self) -> f64 {
        if self.min.len() > self.max.len() {
            return self.min.peek().unwrap().0 as f64;
        } else if self.max.len() > self.min.len() {
            return *self.max.peek().unwrap() as f64;
        }
        (self.min.peek().unwrap().0 + self.max.peek().unwrap()) as f64 / 2.0
    }
}
