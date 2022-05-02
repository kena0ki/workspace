// template

use std::io::{BufRead, BufWriter, Write};
#[allow(unused)]
use std::collections::*;

use std::{collections::btree_set::IntoIter, iter::StepBy};

use std::collections::{BTreeSet, HashMap};

#[derive(Debug,Default,Copy,Clone,PartialEq,Eq)]
pub struct Edge {
    pub u: usize,
    pub v: usize,
}

#[derive(Debug,Default,Copy,Clone,PartialEq,Eq)]
pub struct WeightedEdge {
    pub u: usize,
    pub v: usize,
    pub weight: i64,
}

#[derive(Debug,Default,Copy,Clone,PartialEq,Eq)]
pub struct FlowEdge {
    pub u: usize,
    pub v: usize,
    pub cap: i64,
    pub cost: i64,
    pub flow: i64,
}

#[derive(Debug,Default,Copy,Clone,PartialEq,Eq)]
pub struct AdjTo {
    pub edge_id: usize,
    pub v: usize,
}
impl Ord for AdjTo {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        return self.edge_id.cmp(&other.edge_id);
    }
}
impl PartialOrd for AdjTo {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

/// A compact graph representation.
#[derive(Debug,Default,Clone,PartialEq,Eq)]
pub struct Graph<T> {
    adj: HashMap<usize,BTreeSet<AdjTo>>, // two edges for an undirected edge
    num_vert: usize,
    edges: Vec<T>, // one edge for an undirected edge
}

impl <T:std::fmt::Debug> Graph<T> {
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

    /// Gets vertex u's adjacency list.
    pub fn adj_list(&self, u: usize) -> BTreeSet<AdjTo> {
        return self.adj.get(&u).unwrap_or(&BTreeSet::new()).to_owned();
    }

    /// Gets an edge
    pub fn edge(&self, edge_id: usize) -> &T {
        return &self.edges[edge_id];
    }

    pub fn debug_print(&self) {
        for e in &self.edges {
            println!("{:?}", e);
        }
    }
}

impl Graph<FlowEdge> {
    pub fn add_flow_edge(&mut self, u: usize, v: usize, cap: i64, rcap: i64, cost: i64) {
        let edge_id = self.num_e();
        // add an edge
        self.adj.entry(u).or_default().insert(AdjTo{ edge_id, v });
        self.edges.push(FlowEdge { u, v, cap, cost, flow:0 });
        // add a residual edge
        self.adj.entry(v).or_default().insert(AdjTo{ edge_id: edge_id+1, v:u });
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
    const INF: i64 = i64::max_value();

    /// Initializes an flow network with vmax vertices and no edges.
    pub fn new(vmax: usize, emax_hint: usize) -> Self {
        Self {
            graph: Graph::new(vmax, 2 * emax_hint),
            distance: vec![],
        }
    }

    /// Adds an edge with specified directional capacities and cost per unit of
    /// flow. If only forward flow is allowed, rcap should be zero.
    pub fn add_edge(&mut self, u: usize, v: usize, cap: i64, cost: i64) {
        self.graph.add_flow_edge(u,v,cap,0,cost);
    }

    pub fn add_edge_rcap(&mut self, u: usize, v: usize, cap: i64, rcap: i64, cost:i64) {
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
            for AdjTo{edge_id:e, v} in self.graph.adj_list(u) {
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
        adj: &mut [::std::iter::Peekable<IntoIter<AdjTo>>],
    ) -> i64 {
        if u == t {
            return flow_input;
        }
        let mut flow_used = 0;

        while let Some(&AdjTo{edge_id:e, v}) = adj[u].peek() {
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
        return self.mcf_flow_limit(s,t,i64::max_value());
    }

    /// Finds minimum cost flow with a flow limitation.
    pub fn mcf_flow_limit(&mut self, s: usize, t: usize, mut flow_limit: i64) -> (i64, i64) {
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
            if flow_limit <= 0 {
                break;
            }
            let par = self.mcf_search(s, &mut pot); // find shortest path from s to t.
            if par[t] == None {
                break;
            }
            let (dc, df) = self.mcf_augment(t, &par,flow_limit);
            min_cost += dc;
            max_flow += df;
            flow_limit-=df;
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
            for AdjTo{edge_id:e, v} in self.graph.adj_list(u) {
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
    fn mcf_augment(&mut self, t: usize, par: &[Option<usize>], flow_limit: i64) -> (i64, i64) {
        let (mut dc, mut df) = (0, Self::INF);
        let mut u = t;
        while let Some(e) = par[u] {
            let edge = &self.graph.edges[e];
            df = df.min(edge.cap - edge.flow).min(flow_limit);
            u = edge.u;
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

    pub fn debug_print(&self, residual: bool) {
        let step = if residual { 1 } else { 2 };
        for e in self.graph.edges.iter().step_by(step) {
            println!("{:?}", e);
        }
    }
}

fn main() {
    let sin = std::io::stdin();
    let scan = &mut Scanner::new(sin.lock());
    let sout = std::io::stdout();
    let out = &mut BufWriter::new(sout.lock());
    solve(scan, out);
}

#[allow(unused)]
#[macro_export]
macro_rules! logln {
    ($($arg:tt)*) => ({
        #[cfg(debug_assertions)]
        println!($($arg)*);
    })
}

pub struct Scanner<R> {
    reader: R,
    buffer: Vec<String>,
}
impl<R: ::std::io::BufRead> Scanner<R> {
    pub fn new(reader: R) -> Self {
        Self { reader, buffer: vec![] }
    }
    pub fn token<T: ::std::str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buffer.pop() {
                return token.parse().ok().expect("Failed parse");
            }
            let mut input = String::new();
            self.reader.read_line(&mut input).expect("Failed read");
            self.buffer = input.split_whitespace().rev().map(String::from).collect();
        }
    }
    pub fn token_bytes(&mut self) -> Vec<u8> {
        let s = self.token::<String>();
        return s.as_bytes().into();
    }
}

#[cfg(test)]
mod abc999x {
    use super::*;

    macro_rules! test_macro {
        ($name:ident, $input:expr, $expected:expr) => {
            #[test]
            fn $name() {
                let output = &mut Vec::new();
                let scan = &mut Scanner::new($input as &[u8]);
                solve(scan, output);
                assert_eq!($expected, std::str::from_utf8(output).unwrap());
            }
        };
    }

    test_macro!(test0, b"\
7 7 1
10 0 0  0 0 0 10
 0 0 0  0 0 0 0
 0 0 0  0 0 0 0
 0 0 0 10 0 0 0
 0 0 0  0 0 0 0
 0 0 0  0 0 0 0
 0 0 0  0 0 0 0
" , "\
32
");

    test_macro!(test1, b"\
2 2 2
2 10
8 3
" , "\
12
");

    test_macro!(test2, b"\
3 3 100
1 1 1
1 1 1
1 1 1
" , "\
0
");

    test_macro!(test3, b"\
8 9 970861213
1313462 943495812 203775264 839015475 115668311 14701110 819458175 827176922 236492592
843915104 786367010 344840288 618248834 824858165 549189141 120648070 805825275 933750119
709330492 38579914 890555497 75314343 238373458 854061807 637519536 53226153 627677130
671706386 380984116 221773266 787763728 639374738 298691145 359138139 183373508 524415106
716502263 150803008 390520954 913021901 553285119 876389099 952721235 46809105 635239775
355621458 511843148 117663063 37274476 891025941 832254337 346436418 783134705 488516288
383723241 322408013 948364423 409068145 120813872 697127655 968230339 988041557 222591780
712959990 233114128 210373172 798667159 568746366 579461421 923556823 777007925 422249456
" , "\
9785518299
");

}

// https://atcoder.jp/contests/abc225/tasks/abc225_g
fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let h = scan.token::<usize>();
    let w = scan.token::<usize>();
    let c = scan.token::<i64>();
    let s = h*w;
    let t = s+1;
    let n = s+2;
    let mut mf = FlowGraph::new(n,2*h*w);
    let mut ans = 0;
    for i in 0..h { for j in 0..w {
        let a = scan.token::<i64>();
        let v = i*w + j;
        mf.add_edge(v, t, a, 0);
        ans += a;
    } }
    for i in 0..h { for j in 0..w {
        let v = i*w + j;
        if i>=1 && j>=1 {
            let u = v-w-1;
            mf.add_edge(u, v, c, 0);
        } else {
            mf.add_edge(s, v, c, 0);
        }
        if i>=1 && j<w-1 {
            let u = v-w+1;
            mf.add_edge(u, v, c, 0);
        } else {
            mf.add_edge(s, v, c, 0);
        }
    } }
    let f = mf.dinic(s,t);
    mf.debug_print(false);
    ans -= f;
    writeln!(out, "{}a", ans).ok();
}

