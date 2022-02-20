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
    const MXE:usize = 100_000;
    let n = scan.token::<usize>();
    let mut t = vec![0;n];
    let mut adj = vec![Vec::<usize>::with_capacity(MXE);n];
    for i in 0..n {
        t[i] = scan.token::<usize>();
        let k = scan.token::<usize>();
        for _ in 0..k{
            let v = scan.token::<usize>()-1;
            adj[i].push(v);
        }
    }

    let mut vis = vec![false;n];
    let ans = dfs(&adj,n-1,&t, &mut vis);
    fn dfs(adj: &Vec<Vec<usize>>, u:usize, t:&Vec<usize>, vis: &mut Vec<bool>) -> usize{
        let mut total = t[u];
        vis[u]=true;
        for &v in &adj[u] {
            if vis[v] == false {
                total += dfs(adj,v,t,vis);
            }
        }
        return total;
    }
    writeln!(out, "{}", ans).ok();
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
mod abc226c {
    use super::*;

    #[test]
    fn test1() {
        let input: &[u8] = b"\
3
3 0
5 1 1
7 1 1
";
        let expected = "\
10
";
        let output = &mut Vec::new();
        let scan = &mut scanner::Scanner::new(input);
        solve(scan, output);

        assert_eq!(expected, std::str::from_utf8(output).unwrap());
    }

    #[test]
    fn test2() {
        let input: &[u8] = b"\
5
1000000000 0
1000000000 0
1000000000 0
1000000000 0
1000000000 4 1 2 3 4
";
        let expected = "\
5000000000
";
        let output = &mut Vec::new();
        let scan = &mut scanner::Scanner::new(input);
        solve(scan, output);

        assert_eq!(expected, std::str::from_utf8(output).unwrap());
    }
}
