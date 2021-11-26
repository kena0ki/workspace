use text_io::read;
use disjoint::DisjointSets;

//https://atcoder.jp/contests/practice2/tasks/practice2_a
//
// input:
// 4 7
// 1 0 1
// 0 0 1
// 0 2 3
// 1 0 1
// 1 1 2
// 0 0 2
// 1 1 3
// expected:
// 0
// 1
// 0
// 1
//
fn main() {
    let n:usize = read!();
    let q:usize = read!();
    let mut qs:Vec<(usize,usize,usize)> = Vec::with_capacity(q);
    for _ in 0..q {
        let o:usize = read!();
        let u:usize = read!();
        let v:usize = read!();
        qs.push((o,u,v));
    }
    let mut ds = DisjointSets::new(n);
    for query in qs {
        if query.0 == 0 {
            ds.merge(query.1, query.2);
        } else {
            if ds.find(query.1) == ds.find(query.2) {
                println!("connected");
            } else {
                println!("not connected");
            }
        }
    }
    println!("{:?}", ds);
}

pub mod disjoint {
    /// Represents a union of disjoint sets. Each set's elements are arranged in a
    /// tree, whose root is the set's representative.
    #[derive(Debug)]
    pub struct DisjointSets {
        parent: Vec<usize>,
    }

    impl DisjointSets {
        /// Initializes disjoint sets containing one element each.
        pub fn new(size: usize) -> Self {
            Self {
                parent: (0..size).collect(),
            }
        }

        /// Finds the set's representative. Do path compression along the way to make
        /// future queries faster.
        pub fn find(&mut self, u: usize) -> usize {
            let pu = self.parent[u];
            if pu != u {
                self.parent[u] = self.find(pu);
            }
            self.parent[u]
        }

        /// Merges the sets containing u and v into a single set containing their
        /// union. Returns true if u and v were previously in different sets.
        pub fn merge(&mut self, u: usize, v: usize) -> bool {
            let (pu, pv) = (self.find(u), self.find(v));
            self.parent[pu] = pv;
            pu != pv
        }
    }
}


