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
1 0 1
1
" , "\
1
");

    test_macro!(test1, b"\
5 6 3
4 2
4 3
1 2
2 3
4 5
1 5
1 3 4
" , "\
1 3 3 3 3
");

    test_macro!(test2, b"\
14 14 8
7 4
13 9
9 8
4 3
7 2
13 8
12 8
11 3
6 3
7 14
6 5
1 4
10 13
5 2
2 6 12 9 1 10 5 4
" , "\
1 6 1 1 6 6 1 9 9 10 11 12 10 14
");

}

// https://atcoder.jp/contests/abc219/tasks/abc219_g
fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let m = scan.token::<usize>();
    let q = scan.token::<usize>();
    let mut adj = vec![Vec::new();n];
    let mut va = vec![(0,0);n];
    for i in 0..n {
        va[i] = (0,i+1);
    }
    let mut vdeg = vec![0;n];
    for _ in 0..m {
        let u = scan.token::<usize>()-1;
        let v = scan.token::<usize>()-1;
        adj[u].push(v);
        adj[v].push(u);
        vdeg[u] += 1;
        vdeg[v] += 1;
    }
    let mut adj2 = vec![Vec::new();n];
    for i in 0..n {
        let deg = vdeg[i];
        if deg*deg < m { continue; }
        for &v in &adj[i] {
            let deg = vdeg[v];
            if deg*deg < m { continue; }
            adj2[i].push(v);
        }
    }
    let mut vb:Vec<Option<(usize,usize)>> = vec![None;n];
    for i in 0..q {
        let x = scan.token::<usize>()-1;
        let deg = vdeg[x];
        if deg*deg < m {
            let mut latest = va[x];
            for &v in &adj[x] {
                if vb[v].is_none() { continue; }
                let b = vb[v].unwrap();
                if latest.0 < b.0 {
                    latest = b;
                }
            }
            va[x] = (i+1, latest.1);
            for &v in &adj[x] {
                va[v] = va[x];
            }
        } else {
            vb[x] = Some((i+1, va[x].1));
            for &v in &adj2[x] {
                va[v] = va[x];
            }
        }
    }
    for i in 0..n {
        let x = i;
        let deg = vdeg[x];
        if deg*deg < m {
            let mut latest = va[x];
            for &v in &adj[x] {
                if vb[v].is_none() { continue; }
                let b = vb[v].unwrap();
                if latest.0 < b.0 {
                    latest = b;
                }
            }
            va[x] = latest;
        }
    }
    for i in 0..n {
        let sp = b" \n"[(i==n-1) as usize];
        write!(out, "{}{}", va[i].1, sp as char).ok();
    }
}

