use text_io::read;
use std::collections::{HashMap,BTreeSet};

//
// https://en.wikipedia.org/wiki/Depth-first_search
// DFS postordering recursive
//
// graph:
//       1
//      / \
//     2   3
//    / \ / \
//   4   5   6
//
// input:
// 6 6
// 1 2
// 1 3
// 2 4
// 2 5
// 3 6
// 3 5
// expected:
// 4 5 2 6 3 1
fn main() {
    let num_vert: usize = read!();
    let num_edges: usize = read!();
    let mut adj:HashMap<usize,BTreeSet<usize>> = HashMap::with_capacity(num_edges);
    for _ in 0..num_edges {
        let u: usize = read!();
        let v: usize = read!();
        adj.entry(u).or_default().insert(v);
    }
    let mut dfs = Dfs::new(adj, num_vert);
    let result = dfs.do_dfs();
    println!("{:?}", result);
}

struct Dfs {
    adj: HashMap<usize,BTreeSet<usize>>,
    num_vert: usize,
    visited: usize,
    result: Vec<usize>,
}

impl Dfs {
    pub fn new(adj: HashMap<usize,BTreeSet<usize>>, num_vert:usize) -> Self {
        return Self {
            adj,
            num_vert,
            visited: 0,
            result: vec![],
        }
    }
    fn traverse(&mut self, node: usize){
        let clone = self.adj.entry(node).or_default().clone();
        for &v in clone.iter() {
            if (1<<v) & self.visited == 0 {
                self.traverse(v);
            }
        }
        self.visited |= 1<<node;
        self.result.push(node);
    }
    pub fn do_dfs(&mut self) -> Vec<usize> {
        for i in 1..self.num_vert+1 {
            if (1<<i) & self.visited == 0 {
                self.traverse(i);
            }
        }
        return self.result.to_vec();
    }
}



