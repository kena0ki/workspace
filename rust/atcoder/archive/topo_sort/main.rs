use std::collections::{HashMap,BTreeSet,BinaryHeap};

use rustrithm::scanner::Scanner;

// https://atcoder.jp/contests/abc223/tasks/abc223_d
// topological sort
// binary heap priority que
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
    let sin = std::io::stdin();
    let mut scan = Scanner::new(sin.lock());
    let num_vert: usize = scan.token();
    let num_edges: usize = scan.token();
    let mut adj:HashMap<usize,BTreeSet<usize>> = HashMap::with_capacity(num_edges);
    let mut in_degree:HashMap<usize,usize> = HashMap::new();
    for _ in 0..num_edges {
        let u: usize = scan.token();
        let v: usize = scan.token();
        adj.entry(u).or_default().insert(v);
        *in_degree.entry(v).or_default() += 1;
    }
    let mut heap = BinaryHeap::with_capacity(num_vert);
    for i in 1..num_vert+1 {
        if ! in_degree.contains_key(&i) {
            heap.push(OrdUsize::new(i));
        }
    }
    let mut result: Vec<usize> = Vec::with_capacity(num_vert);
    while let Some(vert) = heap.pop() {
        result.push(vert.val);
        for v in adj.entry(vert.val).or_default().iter() {
            let deg = in_degree.get_mut(v).unwrap();
            *deg -= 1;
            if *deg == 0 {
                in_degree.remove(v);
                heap.push(v.into());
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

#[derive(Copy, Clone, Eq, PartialEq)]
struct OrdUsize {
    val: usize,
}

impl OrdUsize {
    pub fn new(val: usize) -> Self {
        return Self { val };
    }
}

impl From<&mut usize> for OrdUsize {
    fn from(val: &mut usize) -> Self {
        return Self { val: *val };
    }
}

impl From<&usize> for OrdUsize {
    fn from(val: &usize) -> Self {
        return Self { val: *val };
    }
}

impl From<usize> for OrdUsize {
    fn from(val: usize) -> Self {
        return Self { val };
    }
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for OrdUsize {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        return other.val.cmp(&self.val);
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for OrdUsize {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
