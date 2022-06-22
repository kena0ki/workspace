// template

use std::io::{BufRead, BufWriter, Write};
use rustrithm::{scanner, graph::{Graph, Edge}, math::modulo::ModU64};

fn main() {
    let sin = std::io::stdin();
    let scan = &mut scanner::Scanner::new(sin.lock());
    let sout = std::io::stdout();
    let out = &mut BufWriter::new(sout.lock());
    solve(scan, out);
}

const MOD:u64 = 1000000007;
fn solve(scan: &mut scanner::Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let mut graph = Graph::<Edge>::new(n,n-1);
    for _ in 0..n-1 {
        let u = scan.token::<usize>() -1;
        let v = scan.token::<usize>() -1;
        graph.add_undirected_edge(u,v);
    }
    let mut dp = vec![vec![ModU64::<MOD>::new(0);2];n];
    dfs(&graph, 0, None, &mut dp);
    fn dfs(graph: &Graph<Edge>, u:usize, p:Option<usize>, dp: &mut Vec<Vec<ModU64<MOD>>>) {
        let mut white = ModU64::<MOD>::new(1);
        let mut black = ModU64::<MOD>::new(1);
        for a in graph.adj_list(u) {
            if p.is_some() && a.v == p.unwrap() {
                continue;
            }
            dfs(graph, a.v, Some(u), dp);
            white *= dp[a.v][0] + dp[a.v][1];
            black *= dp[a.v][0];
        }
        dp[u][0] = white;
        dp[u][1] = black;
    }
    writeln!(out, "{}", dp[0][0]+dp[0][1]).ok();
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
mod edpc_p {
    use super::*;

    #[test]
    fn test1() {
        let input: &[u8] = b"\
3
1 2
2 3
";
        let expected = "\
5
";
        let output = &mut Vec::new();
        let scan = &mut scanner::Scanner::new(input);
        solve(scan, output);

        assert_eq!(expected, std::str::from_utf8(output).unwrap());
    }

    #[test]
    fn test2() {
        let input: &[u8] = b"\
4
1 2
1 3
1 4
";
        let expected = "\
9
";
        let output = &mut Vec::new();
        let scan = &mut scanner::Scanner::new(input);
        solve(scan, output);

        assert_eq!(expected, std::str::from_utf8(output).unwrap());
    }

    #[test]
    fn test3() {
        let input: &[u8] = b"\
1
";
        let expected = "\
2
";
        let output = &mut Vec::new();
        let scan = &mut scanner::Scanner::new(input);
        solve(scan, output);

        assert_eq!(expected, std::str::from_utf8(output).unwrap());
    }

    #[test]
    fn test4() {
        let input: &[u8] = b"\
10
8 5
10 8
6 5
1 5
4 8
2 10
3 6
9 2
1 7
";
        let expected = "\
157
";
        let output = &mut Vec::new();
        let scan = &mut scanner::Scanner::new(input);
        solve(scan, output);

        assert_eq!(expected, std::str::from_utf8(output).unwrap());
    }
}
