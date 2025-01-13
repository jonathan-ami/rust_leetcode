use std::cmp::min;

pub struct Solution {}

impl Solution {
    pub fn network_delay_time(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32 {
        use std::cmp::Reverse;
        use std::collections::BinaryHeap;
        use std::collections::HashMap;
        use std::collections::HashSet;

        #[derive(PartialEq, Eq, PartialOrd, Ord)]
        pub struct Edge {
            weight: i32,
            node: i32,
        }

        let mut visited: HashSet<i32> = HashSet::new();
        let mut edges: HashMap<i32, Vec<Edge>> = HashMap::new();

        for val in times {
            let (u, v, w) = (val[0], val[1], val[2]);

            let edge = Edge { weight: w, node: v };

            if edges.contains_key(&u) {
                edges.entry(u).and_modify(|val| val.push(edge));
            } else {
                edges.insert(u, vec![edge]);
            }
        }

        let mut min_heap: BinaryHeap<Reverse<Edge>> = BinaryHeap::new();
        let mut res = 0;

        min_heap.push(Reverse(Edge { weight: 0, node: k }));

        while !min_heap.is_empty() {
            let edge = min_heap.pop().unwrap().0;

            if visited.contains(&edge.node) {
                continue;
            }
            visited.insert(edge.node);
            res = edge.weight;

            if let Some(edges) = edges.get(&edge.node) {
                for e in edges {
                    if !visited.contains(&e.node) {
                        min_heap.push(Reverse(Edge {
                            weight: e.weight + res,
                            node: e.node,
                        }));
                    }
                }
            }
        }

        if res == 0 || visited.len() != n as usize {
            return -1;
        }
        res
    }
}
