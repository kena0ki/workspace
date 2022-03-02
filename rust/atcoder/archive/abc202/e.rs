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
7
1 1 2 2 4 2
4
1 2
7 2
4 1
5 5
" , "\
3
1
0
0
");

}

// https://atcoder.jp/contests/abc202/tasks/abc202_f
fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let mut adj = vec![Vec::new();n];
    for i in 0..n-1 {
        let p = scan.token::<usize>()-1;
        adj[p].push(i+1);
    }
    logln!("{:?}",adj);
    let mut vu = vec![Vec::new();n];
    let mut vin = vec![0;n];
    let mut vout = vec![0;n];
    f(0,&adj, &mut vu, 0, 0, &mut vin, &mut vout);
    fn f(u:usize, adj: &Vec<Vec<usize>>, vu: &mut Vec<Vec<usize>>, d: usize,
        mut p:usize, vin: &mut Vec<usize>, vout: &mut Vec<usize>) -> usize {
        vu[d].push(p);
        vin[u] = p;
        for &v in &adj[u] {
            p = f(v,adj,vu,d+1,p+1, vin,vout);
        }
        vout[u] = p;
        return p;
    }
    for i in 0..n {
        vu[i].sort_unstable();
    }
    logln!("{:?}", vu);
    logln!("{:?}", vin);
    logln!("{:?}", vout);
    let q = scan.token::<usize>();
    for _ in 0..q {
        let u = scan.token::<usize>()-1;
        let d = scan.token::<usize>();
        let start = vu[d].binary_search_by_key(&vin[u], |&a| a);
        let start = start.map_or_else(|v| v, |v| v);
        let end = vu[d].binary_search_by_key(&vout[u], |&a| a);
        let end = end.map_or_else(|v| v, |v| v+1);
        let ans = end - start;
        writeln!(out, "{}a",ans).ok();
    }

}

