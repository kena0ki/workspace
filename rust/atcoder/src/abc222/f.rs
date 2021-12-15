// template

use std::io::{BufRead, BufWriter, Write};
use rustrithm::{scanner, graph::{Graph, WeightedEdge}};

fn main() {
    let sin = std::io::stdin();
    let scan = &mut scanner::Scanner::new(sin.lock());
    let sout = std::io::stdout();
    let out = &mut BufWriter::new(sout.lock());
    let mut s = Solver::default();
    s.solve(scan, out);
}

#[derive(Default)]
struct Solver {
    graph: Graph::<WeightedEdge>,
    d: Vec<i64>,
}
impl Solver {
    fn solve(&mut self, scan: &mut scanner::Scanner<impl BufRead>, out: &mut impl Write) {
        let n = scan.token::<usize>();
        self.graph = Graph::<WeightedEdge>::new(n, n-1);
        for _ in 0..n-1 {
            let u = scan.token::<usize>()-1;
            let v = scan.token::<usize>()-1;
            let w = scan.token::<i64>();
            self.graph.add_weighted_undirected_edge(u,v,w);
        }
        self.d = vec![0;n];
        for i in 0..n {
            self.d[i] = scan.token::<i64>();
        }
        let (_, s) = self.dfs(0,None,0,0);
        let (_, t) = self.dfs(s,None,0,0);
        logln!("{},{}", s,t);
        let mut max = vec![0;n];
        self.dfs2(s,None, 0, self.d[s], &mut max);
        logln!("max:{:?}", max);
        self.dfs2(t,None, 0, self.d[t], &mut max);
        for i in 0..n {
            writeln!(out, "{}", max[i]).ok();
        }
    }
    fn dfs(&self, u:usize, p:Option<usize>, c:i64, d:i64) -> (i64, usize) {
        let mut ret = (c+d,u);
        for adj in self.graph.adj_list(u) {
            if p.is_some() && adj.v == p.unwrap() {
                continue;
            }
            let cost = self.graph.edge(adj.edge_id).weight;
            ret = ret.max(self.dfs(adj.v, Some(u), c+cost, self.d[adj.v]));
        }
        return ret;
    }
    fn dfs2(&self, u:usize, p:Option<usize>, c:i64, d:i64, max: &mut Vec<i64>) {
        for adj in self.graph.adj_list(u) {
            if p.is_some() && adj.v == p.unwrap() {
                continue;
            }
            let cost = self.graph.edge(adj.edge_id).weight;
            max[adj.v]=(c+cost+d).max(max[adj.v]);
            self.dfs2(adj.v, Some(u), c+cost, d, max);
        }
    }
}

#[allow(unused)]
#[macro_export]
macro_rules! logln {
    ($($arg:tt)*) => ({
        #[cfg(debug_assertions)]
        println!($($arg)*);
    })
}

#[cfg(test)]
mod abc222f {
    use super::*;

    #[test]
    fn test1() {
        let input: &[u8] = b"\
3
1 2 2
2 3 3
1 2 3
";
        let expected = "\
8
6
6
";
        let output = &mut Vec::new();
        let scan = &mut scanner::Scanner::new(input);
        Solver::default().solve(scan, output);

        assert_eq!(expected, std::str::from_utf8(output).unwrap());
        println!();
    }

    #[test]
    fn test2() {
        let input: &[u8] = b"\
6
1 2 3
1 3 1
1 4 4
1 5 1
1 6 5
9 2 6 5 3 100
";
        let expected = "\
105
108
106
109
106
14
";
        let output = &mut Vec::new();
        let scan = &mut scanner::Scanner::new(input);
        Solver::default().solve(scan, output);

        assert_eq!(expected, std::str::from_utf8(output).unwrap());
        println!();
    }

    #[test]
    fn test3() {
        let input: &[u8] = b"\
6
1 2 1000000000
2 3 1000000000
3 4 1000000000
4 5 1000000000
5 6 1000000000
1 2 3 4 5 6
";
        let expected = "\
5000000006
4000000006
3000000006
3000000001
4000000001
5000000001
";
        let output = &mut Vec::new();
        let scan = &mut scanner::Scanner::new(input);
        Solver::default().solve(scan, output);

        assert_eq!(expected, std::str::from_utf8(output).unwrap());
        println!();
    }
}
