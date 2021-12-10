use std::io::{BufRead, BufWriter, Write};
use rustrithm::{scanner, graph::{Graph, Edge}, math::modulo::ModU64};

// https://atcoder.jp/contests/abc221/tasks/abc221_f
fn main() {
    let sin = std::io::stdin();
    let scan = &mut scanner::Scanner::new(sin.lock());
    let sout = std::io::stdout();
    let out = &mut BufWriter::new(sout.lock());
    solve(scan, out);
}

fn solve(scan: &mut scanner::Scanner<impl BufRead>, out: &mut impl Write) {
    const MOD:u64 = 998244353;
    let n = scan.token::<usize>(); // n >= 2 from the constraint
    let graph = &mut Graph::<Edge>::new(n, n-1);
    for _ in 0..n-1 {
        let u = scan.token::<usize>();
        let v = scan.token::<usize>();
        graph.add_undirected_edge(u-1,v-1);
    }
    let (_, s) = dfs1(graph, 0, 1, None);
    let (d, e) = dfs1(graph, s, 1, None);
    fn dfs1(graph: &Graph<Edge>, u: usize, d: usize, p: Option<usize>) -> (usize, usize) {
        let mut ret = (d,u);
        for adj in graph.adj_list(u) {
            if p.is_some() && adj.v == p.unwrap() {
                continue;
            }
            ret=ret.max(dfs1(graph, adj.v, d+1, Some(u)));
        }
        //println!("ret: {:?}", ret);
        return ret;
    }
    let path = &mut Vec::with_capacity(d);
    dfs2(graph, s, e, None, path);
    fn dfs2(graph: &Graph<Edge>, u: usize, e: usize, p: Option<usize>, path: &mut Vec<usize>) -> bool {
        if u==e {
            return true;
        }
        for adj in graph.adj_list(u) {
            if p.is_some() && adj.v == p.unwrap() {
                continue;
            }
            if dfs2(graph, adj.v, e, Some(u), path) {
                //println!("path: {}", adj.v);
                path.push(adj.v);
                return true;
            }
        }
        return false;
    }
    let mut ans = ModU64::<MOD>::new(1);
    let r = d/2;
    if d & 1 == 0 {
        ans *= dfs3(graph, path[r-1], 1, r, path[r]);
        ans *= dfs3(graph, path[r], 1, r, path[r-1]);
    } else {
        let mut sum = ModU64::<MOD>::new(0);
        for adj in graph.adj_list(path[r]) {
            let ret = dfs3(graph, adj.v, 1, r, path[r]);
            ans *= ret + 1;
            sum += ret;
        }
        ans = ans - sum - 1;
    }
    fn dfs3(graph: &Graph<Edge>, u: usize, d: usize, r: usize, p: usize) -> ModU64<MOD> {
        let mut cnt = ModU64::<MOD>::new(0);
        if d==r {
            return ModU64::<MOD>::new(1);
        }
        for a in graph.adj_list(u) {
            if a.v == p {
                continue;
            }
            cnt+=dfs3(graph, a.v, d+1, r, u);
        }
        return cnt;
    }

    writeln!(out, "{}", ans).ok();
}

#[cfg(test)]
mod abc221f {
    use super::*;

    #[test]
    fn test1() {
        let input: &[u8] = b"\
5
1 2
1 3
1 4
4 5
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
    fn test2() {
        let input: &[u8] = b"\
4
1 2
1 3
1 4
";
        let expected = "\
4
";
        let output = &mut Vec::new();
        let scan = &mut scanner::Scanner::new(input);
        solve(scan, output);

        assert_eq!(expected, std::str::from_utf8(output).unwrap());
    }
}
