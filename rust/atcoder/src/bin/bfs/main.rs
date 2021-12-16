use std::collections::{HashMap,BTreeSet,VecDeque};

use rustrithm::scanner::Scanner;

//
// https://en.wikipedia.org/wiki/Breadth-first_search
// BFS
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
// 1 2 3 4 5 6
fn main() {
    let sin = std::io::stdin();
    let mut scan = Scanner::new(sin.lock());
    let num_vert: usize = scan.token();
    let num_edges: usize = scan.token();
    let mut adj:HashMap<usize,BTreeSet<usize>> = HashMap::with_capacity(num_edges);
    for _ in 0..num_edges {
        let u: usize = scan.token();
        let v: usize = scan.token();
        adj.entry(u).or_default().insert(v);
    }
    let mut visited = 0;
    let mut que = VecDeque::new();
    let mut result = Vec::with_capacity(num_vert);
    for i in 1..num_vert+1 {
        if (1 << i) & visited == 0 {
            que.push_back(i);
            while let Some(vert) = que.pop_front() {
                if (1 << vert) & visited > 0 { // if the graph has cycles, vertices are possibly queued afterward.
                    continue;
                }
                for &v in adj.entry(vert).or_default().iter() {
                    que.push_back(v);
                }
                visited |= 1<<vert;
                result.push(vert);
            }
        }
    }
    println!("{}", visited);
    println!("{:?}", result);
}

