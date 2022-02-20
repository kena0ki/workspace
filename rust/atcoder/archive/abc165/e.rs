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
mod abc165e {
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
5 2
" , "\
1 5
2 4
");

    test_macro!(test1, b"\
4 1
" , "\
2 3
");

    test_macro!(test2, b"\
7 3
" , "\
1 6
2 5
3 4
");

}

// https://atcoder.jp/contests/abc165/tasks/abc165_e
fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let m = scan.token::<usize>();
    let m2 = (m + 1) /2;
    let mut a = n;
    let mut b = n-1;
    for _ in 0..m2 {
        a = (a - 1)% n;
        b = (b + 1)% n;
        writeln!(out, "{} {}", a+1,b+1).ok();
    }
    let n2 = n/2;
    let mut a = n2;
    let mut b = n2;
    let m2 = m/2;
    for _ in 0..m2 {
        a = (a - 1)% n;
        b = (b + 1)% n;
        writeln!(out, "{} {}", a+1,b+1).ok();
    }
}
