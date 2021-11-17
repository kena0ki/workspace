use text_io::read;
use std::collections::{HashMap,BTreeSet,VecDeque};

// https://atcoder.jp/contests/abc223/tasks/abc223_d
//
// input:
// 5 4
// 2 1
// 3 4
// 2 4
// 5 3
// expected:
// 2 1 5 3 4
fn main() {
    let num_vert: usize = read!();
    let num_edges: usize = read!();
    let mut adj:HashMap<usize,BTreeSet<usize>> = HashMap::with_capacity(num_edges);
    let mut in_degree:HashMap<usize,usize> = HashMap::new();
    for _ in 0..num_edges {
        let u: usize = read!();
        let v: usize = read!();
        adj.entry(u).or_default().insert(v);
        *in_degree.entry(v).or_default() += 1;
    }

    let mut result: Vec<usize> = Vec::with_capacity(num_vert);
    for i in 1..num_vert+1 {
        let mut que = VecDeque::new();
        que.push_back(i);
        while let Some(vert) = que.pop_front() {
            if ! in_degree.contains_key(&vert) {
                result.push(vert);
                for v in adj.entry(vert).or_default().iter() {
                    let deg = in_degree.get_mut(v).unwrap();
                    if *deg == 1 {
                        in_degree.remove(v);
                        if *v < i { // smaller vertex needs to be added at this time.
                            que.push_back(*v);
                        }
                        //que.push_back(*v);
                    } else {
                        *deg -= 1;
                    }
                }
            }
        }
    }

    if in_degree.is_empty() {
        println!("{:?}", result);
    } else {
        println!("Cycle exists");
        println!("{:?}", in_degree);
    }

}
