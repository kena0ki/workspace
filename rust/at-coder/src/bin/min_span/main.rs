
fn main() {
}


pub mod graph {
    /// Represents a union of disjoint sets. Each set's elements are arranged in a
    /// tree, whose root is the set's representative.
    #[derive(Debug,Default,Clone)]
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

    use std::collections::{HashMap,BTreeSet};

    #[derive(Debug,Default,Copy,Clone,PartialEq,Eq)]
    pub struct Edge {
        u: usize,
        v: usize,
        weight: i64,
    }

    /// A compact graph representation.
    #[derive(Debug,Default,Clone,PartialEq,Eq)]
    pub struct Graph {
        adj: HashMap<usize,BTreeSet<usize>>, // two edges for an undirected edge
        num_vert: usize,
        edges: Vec<Edge>, // one edge for an undirected edge
    }

    impl Graph {
        /// Initializes a graph with vmax vertices and no edges. To reduce
        /// unnecessary allocations, emax_hint should be close to the number of
        /// edges that will be inserted.
        pub fn new(vmax: usize, emax_hint: usize) -> Self {
            Self {
                adj: HashMap::with_capacity(emax_hint),
                num_vert: vmax,
                edges: Vec::with_capacity(emax_hint),
            }
        }

        /// Returns the number of vertices.
        pub fn num_v(&self) -> usize {
            return self.num_vert;
        }

        /// Returns the number of edges.
        pub fn num_e(&self) -> usize {
            return self.edges.len();
        }

        pub fn add_weighted_edge(&mut self, u: usize, v: usize, weight: i64) {
            self.edges.push(Edge { u, v, weight });
            self.adj.entry(u).or_default().insert(v);
        }

        /// Adds a directed edge from u to v.
        pub fn add_edge(&mut self, u: usize, v: usize) {
            self.add_weighted_edge(u,v,0);
        }

        pub fn add_weighted_undirected_edge(&mut self, u: usize, v: usize, weight: i64) {
            self.edges.push(Edge { u, v, weight });
            self.adj.entry(u).or_default().insert(v);
            self.adj.entry(v).or_default().insert(u);
        }

        /// An undirected edge is two directed edges. If edges are added only via
        /// this funcion, the reverse of any edge e can be found at e^1.
        pub fn add_undirected_edge(&mut self, u: usize, v: usize) {
            self.add_weighted_undirected_edge(u,v,0);
        }

        /// Kruskal's minimum spanning tree algorithm on an undirected graph.
        pub fn min_spanning_tree(&self) -> Vec<Edge> {
            let mut edges = self.edges.to_vec();
            edges.sort_unstable_by_key(|&e| e.weight);

            let mut components = DisjointSets::new(self.num_v());
            return edges.into_iter()
                .filter(|&e| components.merge(e.u, e.v))
                .collect();
        }
    }
}


#[cfg(test)]
mod min_span {
    use super::*;

    // https://www.geeksforgeeks.org/kruskals-minimum-spanning-tree-algorithm-greedy-algo-2/
    #[test]
    fn min_spanning_tree() {
        let mut graph = graph::Graph::new(9,14);
        graph.add_weighted_undirected_edge(0, 1, 4 );
        graph.add_weighted_undirected_edge(0, 7, 8 );
        graph.add_weighted_undirected_edge(1, 2, 8 );
        graph.add_weighted_undirected_edge(1, 7, 11);
        graph.add_weighted_undirected_edge(2, 3, 7 );
        graph.add_weighted_undirected_edge(2, 5, 4 );
        graph.add_weighted_undirected_edge(3, 4, 9 );
        graph.add_weighted_undirected_edge(3, 5, 14);
        graph.add_weighted_undirected_edge(5, 4, 10);
        graph.add_weighted_undirected_edge(6, 5, 2 );
        graph.add_weighted_undirected_edge(7, 6, 1 );
        graph.add_weighted_undirected_edge(7, 8, 7 );
        graph.add_weighted_undirected_edge(8, 2, 2 );
        graph.add_weighted_undirected_edge(8, 6, 6 );
        let min_tree = graph.min_spanning_tree();
        println!("{:?}", min_tree);
        let expected = "[Edge { u: 7, v: 6, weight: 1 }, Edge { u: 6, v: 5, weight: 2 }, Edge { u: 8, v: 2, weight: 2 }, Edge { u: 0, v: 1, weight: 4 }, Edge { u: 2, v: 5, weight: 4 }, Edge { u: 2, v: 3, weight: 7 }, Edge { u: 0, v: 7, weight: 8 }, Edge { u: 3, v: 4, weight: 9 }]";
        assert_eq!(expected,format!("{:?}", min_tree));
    }
}

