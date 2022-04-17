// template

use std::io::{BufRead, BufWriter, Write};
#[allow(unused)]
use std::collections::*;

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
3 3 10
1 2 20
2 3 30
1 3 0
" , "\
35
");

    test_macro!(test1, b"\
3 3 10
1 2 20
2 3 30
1 3 45
" , "\
35
");

    test_macro!(test2, b"\
2 2 10
1 2 100
2 2 100
" , "\
-1
");

    test_macro!(test3, b"\
4 5 10
1 2 1
1 4 1
3 4 1
2 2 100
3 3 100
" , "\
0
");

}

// https://atcoder.jp/contests/abc137/tasks/abc137_e
fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let m = scan.token::<usize>();
    let p = scan.token::<i64>();
    let mut adj = Vec::with_capacity(m);
    let mut adj2 = vec![Vec::new();n];
    for _ in 0..m {
        let u = scan.token::<usize>()-1;
        let v = scan.token::<usize>()-1;
        let c = scan.token::<i64>() - p;
        adj.push((u,v,-c));
        adj2[u].push(v);
    }
    let inf = 1<<60;
    let mut dist = vec![inf;n];
    //let mut pre = vec![n;n];
    dist[0]=0;
    for _ in 0..n-1 {
        for &(u,v,c) in &adj {
            if dist[u] == inf { continue; }
            if dist[u] + c < dist[v] {
                dist[v] = dist[u]+c;
                //pre[v] = u;
            }
        }
    }
    for &(u,v,c) in &adj {
        if dist[u] == inf { continue; }
        if dist[u]+c < dist[v] {
            let mut vis = vec![false;n];
            let ng = dfs(&adj2, u, &mut vis, n-1);
            if ng {
                logln!("{},{}",u+1,v+1);
                writeln!(out, "-1").ok();
                return;
            }
        }
    }
    fn dfs(adj2: &Vec<Vec<usize>>, u:usize, vis: &mut Vec<bool>, g: usize) -> bool {
        if vis[u] { return false }
        if u == g { return true; }
        vis[u] = true;
        for &v in &adj2[u] {
            if dfs(adj2,v,vis,g) {
                return true;
            }
        }
        return false;
    }
    let ans = 0.max(-dist[n-1]);
    writeln!(out, "{}", ans).ok();
}

