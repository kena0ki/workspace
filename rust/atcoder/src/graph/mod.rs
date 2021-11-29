//! Basic graph module without explicit support for deletion.
//!
//! # Panics
//!
//! All methods will panic if given an out-of-bounds element index.
pub mod connectivity;
pub mod flow;

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
use std::cmp::Reverse;

#[derive(Debug,Default,Copy,Clone,PartialEq,Eq)]
pub struct Edge {
    u: usize,
    v: usize,
}

#[derive(Debug,Default,Copy,Clone,PartialEq,Eq)]
pub struct WeightedEdge {
    u: usize,
    v: usize,
    weight: i64,
}

#[derive(Debug,Default,Copy,Clone,PartialEq,Eq)]
pub struct InDegree {
    idx: usize,
    v: usize,
}
impl Ord for InDegree {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        return self.idx.cmp(&other.idx);
    }
}
impl PartialOrd for InDegree {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

/// A compact graph representation.
#[derive(Debug,Default,Clone,PartialEq,Eq)]
pub struct Graph<T> {
    adj: HashMap<usize,BTreeSet<InDegree>>, // two edges for an undirected edge
    num_vert: usize,
    edges: Vec<T>, // one edge for an undirected edge
}

impl <T> Graph<T> {
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

    /// Adds a directed edge from u to v.
    fn add_adj(&mut self, u: usize, v: usize) {
        let idx = self.num_e();
        self.adj.entry(u).or_default().insert(InDegree{ idx, v });
    }

    /// An undirected edge is two directed edges. If edges are added only via
    /// this funcion, the reverse of any edge e can be found at e^1.
    fn add_undirected_adj(&mut self, u: usize, v: usize) {
        let idx = self.num_e();
        self.adj.entry(u).or_default().insert(InDegree{ idx, v });
        self.adj.entry(v).or_default().insert(InDegree{ idx, v:u });
    }

    /// Gets vertex u's adjacency list.
    pub fn adj_list(&self, u: usize) -> BTreeSet<InDegree> {
        return self.adj.get(&u).unwrap_or(&BTreeSet::new()).to_owned();
    }
}

impl Graph<Edge> {
    pub fn add_edge(&mut self, u: usize, v: usize) {
        self.add_adj(u,v);
        self.edges.push(Edge { u, v });
    }

    pub fn add_undirected_edge(&mut self, u: usize, v: usize) {
        self.add_undirected_adj(u,v);
        self.edges.push(Edge { u, v });
    }

    /// If we think of each even-numbered vertex as a variable, and its
    /// odd-numbered successor as its negation, then we can build the
    /// implication graph corresponding to any 2-CNF formula.
    /// Note that u||v == !u -> v == !v -> u.
    pub fn add_two_sat_clause(&mut self, u: usize, v: usize) {
        self.add_edge(u ^ 1, v);
        self.add_edge(v ^ 1, u);
    }
}

impl Graph<WeightedEdge> {
    pub fn add_weighted_edge(&mut self, u: usize, v: usize, weight: i64) {
        self.add_adj(u,v);
        self.edges.push(WeightedEdge { u, v, weight });
    }

    pub fn add_weighted_undirected_edge(&mut self, u: usize, v: usize, weight: i64) {
        self.add_undirected_adj(u,v);
        self.edges.push(WeightedEdge { u, v, weight });
    }

    /// Kruskal's minimum spanning tree algorithm on an undirected graph.
    pub fn min_spanning_tree(&self) -> Vec<WeightedEdge> {
        let mut edges = self.edges.to_vec();
        edges.sort_unstable_by_key(|&e| e.weight);

        let mut components = DisjointSets::new(self.num_v());
        return edges.into_iter()
            .filter(|&e| components.merge(e.u, e.v))
            .collect();
    }

    // Single-source shortest paths on a graph with non-negative weights
    pub fn dijkstra(&self, u: usize) -> (Vec<usize>, HashMap<usize,usize>) {
        let mut distance = vec![usize::max_value(); self.num_v()];
        let mut prev = HashMap::with_capacity(self.num_v());
        let mut heap = std::collections::BinaryHeap::new();

        distance[u] = 0;
        heap.push((Reverse(0), 0));
        while let Some((Reverse(distance_u), u)) = heap.pop() {
            if distance[u] < distance_u || self.adj.get(&u).is_none() {
                continue;
            }
            let deg = self.adj.get(&u).unwrap();
            for &InDegree{idx, v} in deg.iter() {
                let distance_v = distance_u + self.edges[idx].weight as usize;
                if distance[v] > distance_v {
                    prev.insert(v,u);
                    distance[v] = distance_v;
                    heap.push((Reverse(distance_v), v));
                }
            }
        }
        return (distance, prev);
    }
}

#[cfg(test)]
mod graph_test {
    use std::collections::VecDeque;

    use super::*;

    // https://www.geeksforgeeks.org/kruskals-minimum-spanning-tree-algorithm-greedy-algo-2/
    #[test]
    fn min_spanning_tree() {
        let mut graph = Graph::new(9,28);
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
        let expected = "[WeightedEdge { u: 7, v: 6, weight: 1 }, WeightedEdge { u: 6, v: 5, weight: 2 }, WeightedEdge { u: 8, v: 2, weight: 2 }, WeightedEdge { u: 0, v: 1, weight: 4 }, WeightedEdge { u: 2, v: 5, weight: 4 }, WeightedEdge { u: 2, v: 3, weight: 7 }, WeightedEdge { u: 0, v: 7, weight: 8 }, WeightedEdge { u: 3, v: 4, weight: 9 }]";
        assert_eq!(expected,format!("{:?}", min_tree));
    }

    // https://www.geeksforgeeks.org/dijkstras-algorithm-for-adjacency-list-representation-greedy-algo-8/
    #[test]
    fn dijkstra() {
        let mut graph = Graph::new(9,28);
        graph.add_weighted_undirected_edge(0, 1, 4);
        graph.add_weighted_undirected_edge(0, 7, 8);
        graph.add_weighted_undirected_edge(1, 2, 8);
        graph.add_weighted_undirected_edge(1, 7, 11);
        graph.add_weighted_undirected_edge(2, 3, 7);
        graph.add_weighted_undirected_edge(2, 8, 2);
        graph.add_weighted_undirected_edge(2, 5, 4);
        graph.add_weighted_undirected_edge(3, 4, 9);
        graph.add_weighted_undirected_edge(3, 5, 14);
        graph.add_weighted_undirected_edge(4, 5, 10);
        graph.add_weighted_undirected_edge(5, 6, 2);
        graph.add_weighted_undirected_edge(6, 7, 1);
        graph.add_weighted_undirected_edge(6, 8, 6);
        graph.add_weighted_undirected_edge(7, 8, 7);
        let (min_dists, prevs) = graph.dijkstra(0);
        assert_eq!([0, 4, 12, 19, 21, 11, 9, 8, 14], &*min_dists);
        println!("prevs: {:?}", prevs);
        let mut v = 8;
        let mut shortest_path = VecDeque::from([v]);
        while let Some(&prev) = prevs.get(&v) {
            println!("prev: {}",prev);
            shortest_path.push_front(prev);
            v = prev;
        }
        assert_eq!([0, 1, 2, 8], shortest_path.make_contiguous());
    }
}

