pub struct Solution {}
impl Solution {
    pub fn find_order(num_courses: i32, prerequisistes: Vec<Vec<i32>>) -> Vec<i32> {
        use std::collections::{HashMap, HashSet};
        let mut map: HashMap<i32, Vec<i32>> = HashMap::new();
        let mut res: Vec<i32> = Vec::new();
        let mut visited = HashSet::new();
        let mut cycle = HashSet::new();
        for pr in prerequisistes {
            map.entry(pr[0])
                .and_modify(|e| e.push(pr[1]))
                .or_insert(vec![pr[1]]);
        }
        fn dfs(
            map: &mut HashMap<i32, Vec<i32>>,
            course: i32,
            visited: &mut HashSet<i32>,
            cycle: &mut HashSet<i32>,
            res: &mut Vec<i32>,
        ) -> bool {
            if visited.contains(&course) {
                return true;
            }
            if visited.contains(&course) {
                return false;
            }
            let p = map.get(&course);
            if p.is_some() {
                let prereqs = p.unwrap().clone();
                cycle.insert(course);
                for pr in prereqs {
                    if !dfs(map, pr, visited, cycle, res) {
                        return false;
                    }
                }
                cycle.remove(&course);
            }
            visited.insert(course);
            res.push(course);
            return true;
        }
        for i in 0..num_courses {
            if !dfs(&mut map, i, &mut visited, &mut cycle, &mut res) {
                return vec![];
            }
        }
        res
    }
}
