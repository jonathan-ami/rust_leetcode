pub struct Solution {}
impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        use std::collections::{HashMap, HashSet};
        let mut map: HashMap<i32, Vec<i32>> = HashMap::new();
        let mut visited = HashSet::new();
        for pr in prerequisites {
            map.entry(pr[0])
                .and_modify(|e| e.push(pr[1]))
                .or_insert(vec![pr[1]]);
        }
        fn dfs(map: &mut HashMap<i32, Vec<i32>>, course: i32, visited: &mut HashSet<i32>) -> bool {
            if !map.contains_key(&course) {
                return true;
            }
            if visited.contains(&course) {
                return false;
            }
            let prereqs = map.get(&course).unwrap().clone();
            visited.insert(course);
            for pr in prereqs {
                if !dfs(map, pr, visited) {
                    return false;
                }
            }
            visited.remove(&course);
            map.remove(&course);
            return true;
        }
        for i in 0..num_courses {
            if !dfs(&mut map, i, &mut visited) {
                return false;
            }
        }
        true
    }
}
