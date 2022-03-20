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
3
1 3
2 3
" , "\
7
");

    test_macro!(test2, b"\
2
1 2
" , "\
3
");

    test_macro!(test3, b"\
10
5 3
5 7
8 9
1 9
9 10
8 4
7 4
6 10
7 2
" , "\
113
");

}

// https://atcoder.jp/contests/abc173/tasks/abc173_f
fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let mut adj = vec![Vec::with_capacity(n);n];
    for _ in 0..n-1 {
        let mut u = scan.token::<usize>()-1;
        let mut v = scan.token::<usize>()-1;
        if u < v {
            std::mem::swap(&mut u, &mut v);
        }
        adj[u].push(v);
    }
    let mut ans = 0;
    for i in 1..n+1 {
        ans += i*(i+1)/2;
    }
    for i in 0..n {
        let rest = n-i;
        for &v in &adj[i] {
            let sum = rest*(v+1);
            ans -= sum;
        }
    }
    writeln!(out, "{}", ans).ok();
}

