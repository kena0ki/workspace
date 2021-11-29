//! Maximum flows, matchings, and minimum cuts.
use std::{collections::btree_set::IntoIter, iter::StepBy};

use super::{Graph, InDegree};

#[derive(Debug,Default,Copy,Clone,PartialEq,Eq)]
pub struct FlowEdge {
    u: usize,
    v: usize,
    cap: i64,
    cost: i64,
    flow: i64,
}

impl Graph<FlowEdge> {
    pub fn add_flow_edge(&mut self, u: usize, v: usize, cap: i64, rcap: i64, cost: i64) {
        let idx = self.num_e();
        // add an edge
        self.adj.entry(u).or_default().insert(InDegree{ idx, v });
        self.edges.push(FlowEdge { u, v, cap, cost, flow:0 });
        // add a residual edge
        self.adj.entry(v).or_default().insert(InDegree{ idx: idx+1, v:u });
        self.edges.push(FlowEdge { v:u, u:v, cap:rcap, cost: -cost, flow:0 });
    }
}

/// Representation of a network flow problem with (optional) costs.
pub struct FlowGraph {
    /// Owned graph, managed by this FlowGraph object.
    pub graph: Graph<FlowEdge>,
    distance: Vec<i64>,
}

impl FlowGraph {
    /// An upper limit to the flow.
    const INF: i64 = i64::MAX;

    /// Initializes an flow network with vmax vertices and no edges.
    pub fn new(vmax: usize, emax_hint: usize) -> Self {
        Self {
            graph: Graph::new(vmax, 2 * emax_hint),
            distance: vec![],
        }
    }

    /// Adds an edge with specified directional capacities and cost per unit of
    /// flow. If only forward flow is allowed, rcap should be zero.
    pub fn add_edge(&mut self, u: usize, v: usize, cap: i64, rcap: i64, cost: i64) {
        self.graph.add_flow_edge(u,v,cap,rcap,cost);
    }

    /// Iterator of the edges not including residual edges.
    pub fn edge_iter(&self) -> StepBy<std::slice::Iter<FlowEdge>>{
        return self.graph.edges.iter().step_by(2);
    }

    /// Get an nth edge. The specified index corresponds to the order of adding edges.
    pub fn get_edge(&self, n: usize) -> &FlowEdge{
        return &self.graph.edges[n*2];
    }

    /// Underlying edges in the graph including residual edges.
    pub fn edges_including_residual_edges(&self) -> &[FlowEdge]{
        return &*self.graph.edges;
    }

    /// clear flow value once they are calculated.
    pub fn clear_flow(&mut self) {
        for e in self.graph.edges.iter_mut() {
            e.flow = 0;
        }
    }

    fn augment_path(&mut self, e: usize, flow: i64) {
        self.graph.edges[e].flow += flow;
        self.graph.edges[e ^ 1].flow -= flow;
    }

    /// Dinic's algorithm to find the maximum flow from s to t where s != t.
    /// Generalizes the Hopcroft-Karp maximum bipartite matching algorithm.
    /// V^2E in general, min(V^(2/3),sqrt(E))E when all edges are unit capacity,
    /// sqrt(V)E when all vertices are unit capacity as in bipartite graphs.
    ///
    /// # Panics
    ///
    /// Panics if the maximum flow is 2^63 or larger.
    pub fn dinic(&mut self, s: usize, t: usize) -> i64 {
        let mut max_flow = 0;
        loop {
            self.dinic_search(s);
            if self.distance[t] == Self::INF {
                break;
            }
            // Keep track of adjacency lists to avoid revisiting blocked edges.
            let mut adj_iters = (0..self.graph.num_v())
                .map(|u| self.graph.adj_list(u).into_iter().peekable())
                .collect::<Vec<_>>();
            max_flow += self.dinic_augment(s, t, Self::INF, &mut adj_iters);
        }
        max_flow
    }

    // Compute BFS distances to restrict attention to shortest path edges.
    fn dinic_search(&mut self, s: usize) {
        let mut q = ::std::collections::VecDeque::new();
        self.distance = vec![Self::INF; self.graph.num_v()];
        self.distance[s] = 0;
        q.push_back(s);
        while let Some(u) = q.pop_front() {
            for InDegree{idx:e, v} in self.graph.adj_list(u) {
                if self.distance[v] == Self::INF && self.graph.edges[e].flow < self.graph.edges[e].cap {
                    self.distance[v] = self.distance[u] + 1;
                    q.push_back(v);
                }
            }
        }
    }

    // Pushes a blocking flow that increases the residual's s-t distance.
    fn dinic_augment(
        &mut self,
        u: usize,
        t: usize,
        flow_input: i64,
        adj: &mut [::std::iter::Peekable<IntoIter<InDegree>>],
    ) -> i64 {
        if u == t {
            return flow_input;
        }
        let mut flow_used = 0;

        while let Some(&InDegree{idx:e, v}) = adj[u].peek() {
            let edge = &self.graph.edges[e];
            let rem_cap = (edge.cap - edge.flow).min(flow_input - flow_used);// min(remaining capacity, remaining flow)
            if rem_cap > 0 && self.distance[v] == self.distance[u] + 1 {
                // calculates maximum flow in a subtree (max_flow).
                // max_flow never exceeds the remaining flow since rem_cap is not greater than
                // the remaining flow.
                let max_flow = self.dinic_augment(v, t, rem_cap, adj);
                self.augment_path(e, max_flow);
                flow_used += max_flow; // add the maximum flow in a subtree
                if flow_used == flow_input { // until the summary reaches to the input flow.
                    break;
                }
            }
            // The current edge is either saturated or blocked.
            adj[u].next();
        }
        flow_used
    }

    /// After running maximum flow, use this to recover the dual minimum cut.
    pub fn min_cut(&self) -> Vec<usize> {
        (0..self.graph.num_e())
            .filter(|&e| { // filter blocked edges
                let edge = &self.graph.edges[e];
                self.distance[edge.u] < Self::INF && self.distance[edge.v] == Self::INF
            })
            .collect()
    }

    /// Among all s-t maximum flows, finds one with minimum cost, assuming
    /// s != t and no negative-cost cycles.
    ///
    /// # Panics
    ///
    /// Panics if the flow or cost overflow a 64-bit signed integer.
    pub fn mcf(&mut self, s: usize, t: usize) -> (i64, i64) {
        let mut pot = vec![0; self.graph.num_v()];

        // Bellman-Ford deals with negative-cost edges at initialization.
        for _ in 1..self.graph.num_v() {
            for e in 0..self.graph.num_e() {
                let edge = &self.graph.edges[e];
                if edge.cap > 0 {
                    pot[edge.v] = pot[edge.v].min(pot[edge.u] + edge.cost);
                }
            }
        }

        let (mut min_cost, mut max_flow) = (0, 0);
        loop {
            let par = self.mcf_search(s, &mut pot); // find shortest path from t to u.
            if par[t] == None {
                break;
            }
            let (dc, df) = self.mcf_augment(t, &par);
            min_cost += dc;
            max_flow += df;
        }
        (min_cost, max_flow)
    }

    // Maintains Johnson's potentials to prevent negative-cost residual edges.
    // This allows running Dijkstra instead of the slower Bellman-Ford.
    fn mcf_search(&mut self, s: usize, pot: &mut [i64]) -> Vec<Option<usize>> {
        let mut vis = vec![false; self.graph.num_v()];
        self.distance = vec![Self::INF; self.graph.num_v()];
        let mut par = vec![None; self.graph.num_v()];

        self.distance[s] = 0;
        while let Some(u) = (0..self.graph.num_v())
            .filter(|&u| !vis[u] && self.distance[u] < Self::INF)
            .min_by_key(|&u| self.distance[u] - pot[u])
        {
            vis[u] = true;
            pot[u] = self.distance[u];
            for InDegree{idx:e, v} in self.graph.adj_list(u) {
                let edge = &self.graph.edges[e];
                if self.distance[v] > self.distance[u] + edge.cost && edge.flow < edge.cap {
                    self.distance[v] = self.distance[u] + edge.cost;
                    par[v] = Some(e);
                }
            }
        }
        par
    }

    // Pushes flow along an augmenting path of minimum cost.
    fn mcf_augment(&mut self, t: usize, par: &[Option<usize>]) -> (i64, i64) {
        let (mut dc, mut df) = (0, Self::INF);
        let mut u = t;
        let mut i=0;
        while let Some(e) = par[u] {
            let edge = &self.graph.edges[e];
            df = df.min(edge.cap - edge.flow);
            u = edge.u;
            i+=1;
            if i>10 {
                break;
            }

        }
        u = t;
        while let Some(e) = par[u] {
            self.augment_path(e, df);
            let edge = &self.graph.edges[e];
            dc += df * edge.cost;
            u = edge.u;
        }
        (dc, df)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_dinic() {
        let mut graph = FlowGraph::new(5, 5);
        graph.add_edge(0, 1, 3, 0, 0);
        graph.add_edge(1, 2, 2, 0, 0);
        graph.add_edge(1, 3, 2, 0, 0);
        graph.add_edge(2, 4, 2, 0, 0);
        graph.add_edge(3, 4, 2, 0, 0);

        let max = graph.dinic(0, 4);
        println!("max: {:?}", max);
        assert_eq!(max, 3);
    }

    #[test]
    fn test_min_cut() {
        let mut graph = FlowGraph::new(3, 2);
        graph.add_edge(0, 1, 4, 0, 0);
        graph.add_edge(1, 2, 3, 0, 0);

        let max = graph.dinic(0, 2);
        println!("max: {:?}", max);
        assert_eq!(max, 3);
        assert_eq!(&[2], &*graph.min_cut());
    }

    #[test]
    fn test_min_cost_flow() {
        let mut graph = FlowGraph::new(4, 4);
        graph.add_edge(0, 1, 10, 0, -10);
        graph.add_edge(1, 2, 7, 0, 8);
        graph.add_edge(2, 3, 7, 0, 8);
        graph.add_edge(1, 3, 7, 0, 10);

        let (cost, flow) = graph.mcf(0, 3);
        assert_eq!(cost, 18);
        assert_eq!(flow, 10);
    }

    #[test]
    fn test_max_matching() {
        let mut graph = FlowGraph::new(14, 4);

        let source = 0;
        let sink = 13;

        //Vertex indices of "left hand side" of bipartite graph go from [left_start, right_start)
        let left_start = 1;
        //Vertex indices of "right hand side" of bipartite graph go from [right_start, sink)
        let right_start = 7;

        //Initialize source / sink connections; both left & right have 6 nodes
        for lhs_vertex in left_start..left_start + 6 {
            graph.add_edge(source, lhs_vertex, 1, 0, 0);
        }

        for rhs_vertex in right_start..right_start + 6 {
            graph.add_edge(rhs_vertex, sink, 1, 0, 0);
        }

        graph.add_edge(left_start + 0, right_start + 1, 1, 0, 0);
        graph.add_edge(left_start + 0, right_start + 2, 1, 0, 0);
        graph.add_edge(left_start + 2, right_start + 0, 1, 0, 0);
        graph.add_edge(left_start + 2, right_start + 3, 1, 0, 0);
        graph.add_edge(left_start + 3, right_start + 2, 1, 0, 0);
        graph.add_edge(left_start + 4, right_start + 2, 1, 0, 0);
        graph.add_edge(left_start + 4, right_start + 3, 1, 0, 0);
        graph.add_edge(left_start + 5, right_start + 5, 1, 0, 0);

        let flow_amt = graph.dinic(source, sink);
        assert_eq!(flow_amt, 5);

        let mut matched_edges = graph.edge_iter()
            .filter(|&e| e.flow>0 && e.u != source && e.v != sink);
        assert_eq!(FlowEdge { u: 1, v: 8, cap: 1, cost: 0, flow: 1 },  *matched_edges.next().unwrap());
        assert_eq!(FlowEdge { u: 3, v: 7, cap: 1, cost: 0, flow: 1 },  *matched_edges.next().unwrap());
        assert_eq!(FlowEdge { u: 4, v: 9, cap: 1, cost: 0, flow: 1 },  *matched_edges.next().unwrap());
        assert_eq!(FlowEdge { u: 5, v: 10, cap: 1, cost: 0, flow: 1 }, *matched_edges.next().unwrap());
        assert_eq!(FlowEdge { u: 6, v: 12, cap: 1, cost: 0, flow: 1 }, *matched_edges.next().unwrap());

        // //L->R edges in maximum matching
        // let left_right_edges = flow
        //     .into_iter()
        //     .enumerate()
        //     .filter(|&(_e, f)| f > 0)
        //     //map to u->v
        //     .map(|(e, _f)| (graph.graph.edges[e].u, graph.graph.edges[e].v))
        //     //leave out source and sink nodes
        //     .filter(|&(u, v)| u != source && v != sink)
        //     .collect::<Vec<_>>();

        // assert_eq!(
        //     left_right_edges,
        //     vec![(1, 8), (3, 7), (4, 9), (5, 10), (6, 12)]
        // );
    }
}
