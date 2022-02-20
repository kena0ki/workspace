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

// https://atcoder.jp/contests/abc169/tasks/abc169_e
fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let mut va = Vec::with_capacity(n);
    let mut vb = Vec::with_capacity(n);
    for _ in 0..n {
        let a = scan.token::<usize>();
        let b = scan.token::<usize>();
        va.push(a);
        vb.push(b);
    }
    va.sort_unstable();
    vb.sort_unstable();
    if n % 2 == 1 {
        let k = (n+1)/2 -1;
        let ans = vb[k] - va[k] +1;
        writeln!(out, "{}",ans).ok();
    } else {
        let k1 = (n+1)/2 -1;
        let k2 = k1+1;
        let l = va[k1] + va[k2];
        let r = vb[k1] + vb[k2];
        let ans = r - l +1;
        writeln!(out, "{}",ans).ok();
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
mod abc169d {
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
2
1 2
2 3
" , "\
3
");

    test_macro!(test2, b"\
3
100 100
10 10000
1 1000000000
" , "\
9991
");

}
