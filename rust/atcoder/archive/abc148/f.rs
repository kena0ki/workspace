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

    test_macro!(test1, b"\
5 4 1
1 2
2 3
3 4
3 5
" , "\
2
");

    test_macro!(test2, b"\
5 4 5
1 2
1 3
1 4
1 5
" , "\
1
");

    test_macro!(test3, b"\
2 1 2
1 2
" , "\
0
");

    test_macro!(test4, b"\
9 6 1
1 2
2 3
3 4
4 5
5 6
4 7
7 8
8 9
" , "\
5
");

}

// https://atcoder.jp/contests/abc148/tasks/abc148_f
fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let a = scan.token::<usize>()-1;
    let b = scan.token::<usize>()-1;
    let mut adj = vec![Vec::new();n];
    for _ in 0..n-1 {
        let u = scan.token::<usize>()-1;
        let v = scan.token::<usize>()-1;
        adj[u].push(v);
        adj[v].push(u);
    }
    let mut vd = Vec::with_capacity(n);
    dfs1(&adj,b,usize::max_value(), a, &mut vd,0);

    fn dfs1(adj:&Vec<Vec<usize>>, u:usize, p:usize, a:usize, vd:&mut Vec<usize>, d:usize) -> bool {
        if u == a { return true };
        for &v in &adj[u] {
            if v == p { continue; }
            if dfs1(adj,v,u,a,vd,d+1) {
                vd.push(v);
                return true;
            }
        }
        return false;
    }
    vd.push(b);
    logln!("{:?}",vd);
    let len = vd.len();
    let half = len/2;
    let c = vd[half-1];
    let p = vd[half];
    let mut ans = dfs2(&adj,c,p, 0);
    logln!("{}",ans);
    ans = ans + len-half - 1 ;
    writeln!(out, "{}", ans).ok();
    fn dfs2(adj:&Vec<Vec<usize>>, u:usize, p:usize, d:usize) -> usize {
        let mut res = d;
        for &v in &adj[u] {
            if v == p { continue; }
            res = res.max(dfs2(adj,v,u,d+1));
        }
        return res;
    }
}

