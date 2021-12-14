// template

use std::io::{BufRead, BufWriter, Write};
use rustrithm::{scanner, graph::{Graph, Edge, AdjTo}, math::modulo::ModU64};

fn main() {
    let sin = std::io::stdin();
    let scan = &mut scanner::Scanner::new(sin.lock());
    let sout = std::io::stdout();
    let out = &mut BufWriter::new(sout.lock());
    solve(scan, out);
}

const MOD:u64 = 998244353;
fn solve(scan: &mut scanner::Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let m = scan.token::<usize>();
    let k = scan.token::<i64>();
    let graph = &mut Graph::<Edge>::new(n, n-1);
    let mut a = vec![0;m];
    for i in 0..m {
        let ai = scan.token::<usize>() - 1;
        a[i] = ai;
    }
    for _ in 0..n-1 {
        let u = scan.token::<usize>() - 1;
        let v = scan.token::<usize>() - 1;
        graph.add_undirected_edge(u,v);
    }
    let mut c = vec![0; n-1];
    let mut sum = 0;
    for i in 0..a.len()-1 {
        dfs(graph, a[i], a[i+1], None, &mut c, &mut sum);
    }
    fn dfs(graph: &Graph<Edge>, u: usize, e: usize, p: Option<usize>, c: &mut Vec<usize>, sum: &mut i64) -> bool {
        if u == e {
            return true;
        }
        for AdjTo{edge_id, v} in graph.adj_list(u) {
            if p.is_some() && p.unwrap() == v {
                continue;
            }
            if dfs(graph, v, e, Some(u), c, sum) {
                c[edge_id] += 1;
                *sum +=1;
                return true;
            }
        }
        return false;
    }
    let tmp = k+sum;
    if tmp < 0 || tmp & 1 == 1 {
        writeln!(out, "0").ok();
        return;
    }
    let red = tmp as usize / 2;
    // let mut dp = vec![vec![ModU64::<MOD>::new(0);red+1];n];
    // dp[0][0] = ModU64::<MOD>::new(1);
    // for i in 1..n {
    //     let x = c[i-1];
    //     for j in 0..=red {
    //         if j >= x {
    //             dp[i][j] = dp[i-1][j-x] + dp[i-1][j];
    //         } else {
    //             dp[i][j] = dp[i-1][j];
    //         }
    //     }
    // }
    // writeln!(out, "{}", dp[n-1][red]).ok();
    let mut dp = vec![ModU64::<MOD>::new(0); red+1];
    dp[0]=ModU64::<MOD>::new(1);
    for i in 1..n {
        let x = c[i-1];
        for j in (x..=red).rev() {
            dp[j] = dp[j] + dp[j-x];
        }
    }
    writeln!(out, "{}", dp[red]).ok();
}

#[cfg(test)]
mod abc222e {
    use super::*;

    #[test]
    fn test1() {
        let input: &[u8] = b"\
4 5 0
2 3 2 1 4
1 2
2 3
3 4
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
3 10 10000
1 2 1 2 1 2 2 1 1 2
1 2
1 3
";
        let expected = "\
0
";
        let output = &mut Vec::new();
        let scan = &mut scanner::Scanner::new(input);
        solve(scan, output);

        assert_eq!(expected, std::str::from_utf8(output).unwrap());
    }

    #[test]
    fn test3() {
        let input: &[u8] = b"\
10 2 -1
1 10
1 2
2 3
3 4
4 5
5 6
6 7
7 8
8 9
9 10
";
        let expected = "\
126
";
        let output = &mut Vec::new();
        let scan = &mut scanner::Scanner::new(input);
        solve(scan, output);

        assert_eq!(expected, std::str::from_utf8(output).unwrap());
    }

    #[test]
    fn test4() {
        let input: &[u8] = b"\
5 8 -1
1 4 1 4 2 1 3 5
1 2
4 1
3 1
1 5
";
        let expected = "\
2
";
        let output = &mut Vec::new();
        let scan = &mut scanner::Scanner::new(input);
        solve(scan, output);

        assert_eq!(expected, std::str::from_utf8(output).unwrap());
    }
}
