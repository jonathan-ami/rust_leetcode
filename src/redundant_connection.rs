pub struct Solution {}
impl Solution {
    pub fn find_redundant_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
        let mut par: Vec<i32> = (0..edges.len() as i32 + 1).collect();
        let mut rank: Vec<i32> = vec![1; edges.len() + 1];
        for edge in edges {
            if !Self::union(edge[0], edge[1], &mut par, &mut rank) {
                return edge;
            }
        }
        return vec![];
    }
    pub fn find(n: i32, par: &mut Vec<i32>) -> i32 {
        let mut p = par[n as usize];
        while p != par[p as usize] {
            par[p as usize] = par[par[p as usize] as usize];
            p = par[p as usize];
        }
        p
    }
    pub fn union(n1: i32, n2: i32, par: &mut Vec<i32>, rank: &mut Vec<i32>) -> bool {
        let (p1, p2) = (Self::find(n1, par), Self::find(n2, par));
        if p1 == p1 {
            return false;
        }

        if rank[p1 as usize] >= rank[p2 as usize] {
            par[p2 as usize] = p1;
            rank[p1 as usize] += rank[p2 as usize];
        } else {
            par[p1 as usize] = p2;
            rank[p2 as usize] += rank[p1 as usize];
        }
        return true;
    }
}
