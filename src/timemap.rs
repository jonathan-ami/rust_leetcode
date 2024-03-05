use std::collections::HashMap;
pub struct Value {
    pub value: String,
    pub timestamp: i32,
}
pub struct TimeMap {
    pub time_map: HashMap<String, Vec<Value>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl TimeMap {
    pub fn new() -> Self {
        TimeMap {
            time_map: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: String, value: String, timestamp: i32) {
        let entry = self.time_map.entry(key).or_insert_with(Vec::new);
        entry.push(Value { value, timestamp });
    }

    pub fn get(&self, key: String, timestamp: i32) -> String {
        let entry = self.time_map.get(&key);
        match entry {
            Some(val) => Self::binary_search(val, timestamp),
            None => "".to_string(),
        }
    }
    fn binary_search(timestamps: &Vec<Value>, timestamp: i32) -> String {
        let mut lp: i32 = 0;
        let mut rp: i32 = (timestamps.len() as i32) - 1;
        let mut result = "".to_string();
        while lp <= rp {
            let mid = (lp + rp) / 2;
            if timestamp >= timestamps[mid as usize].timestamp {
                result = timestamps[mid as usize].value.clone();
                lp = mid + 1;
            } else {
                rp = mid - 1;
            }
        }
        result
    }
}
