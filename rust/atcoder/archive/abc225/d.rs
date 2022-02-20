// template

use std::io::{BufRead, BufWriter, Write};
use rustrithm::scanner;

fn main() {
    let sin = std::io::stdin();
    let scan = &mut scanner::Scanner::new(sin.lock());
    let sout = std::io::stdout();
    let out = &mut BufWriter::new(sout.lock());
    solve(scan, out);
}

// https://atcoder.jp/contests/abc222/tasks/abc222_a
fn solve(scan: &mut scanner::Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let q = scan.token::<usize>();
    let mut adj:Vec<Option<usize>> = vec![None;n];
    let mut adjrev:Vec<Option<usize>> = vec![None;n];
    for _ in 0..q {
        let op = scan.token::<usize>();
        if op == 1 {
            let u = scan.token::<usize>() -1;
            let v = scan.token::<usize>() -1;
            logln!("{},{}",u,v);
            adj[u] = Some(v);
            adjrev[v] = Some(u);
        } else if op == 2 {
            let u = scan.token::<usize>() -1;
            let v = scan.token::<usize>() -1;
            adj[u] = None;
            adjrev[v] = None;
        } else {
            let x = scan.token::<usize>() -1;
            let mut ans = Vec::with_capacity(n);
            if let Some(u) = adjrev[x] {
                dfs(&mut ans, &adjrev, u);
            }
            dfs2(&mut ans, &adj, x);
            let m = ans.len();
            write!(out, "{} ", m).ok();
            for v in &ans[..m-1] {
                write!(out, "{} ", v+1).ok();
            }
            writeln!(out, "{}", ans[m-1]+1).ok();

            fn dfs(ans: &mut Vec<usize>, adj: &Vec<Option<usize>>, u:usize) {
                if let Some(v) = adj[u] {
                    dfs(ans, adj, v);
                }
                ans.push(u);
            }
            fn dfs2(ans: &mut Vec<usize>, adj: &Vec<Option<usize>>, u:usize) {
                ans.push(u);
                if let Some(v) = adj[u] {
                    dfs2(ans, adj, v);
                }
            }
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
mod abc225d {
    use super::*;

    #[test]
    fn test1() {
        let input: &[u8] = b"\
7 14
1 6 3
1 4 1
1 5 2
1 2 7
1 3 5
3 2
3 4
3 6
2 3 5
2 4 1
1 1 5
3 2
3 4
3 6
";
        let expected = "\
5 6 3 5 2 7
2 4 1
5 6 3 5 2 7
4 1 5 2 7
1 4
2 6 3
";
        let output = &mut Vec::new();
        let scan = &mut scanner::Scanner::new(input);
        solve(scan, output);

        assert_eq!(expected, std::str::from_utf8(output).unwrap());
    }
}
