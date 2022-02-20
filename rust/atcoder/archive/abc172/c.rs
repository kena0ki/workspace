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

// https://atcoder.jp/contests/abc172/tasks/abc172_c
fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let m = scan.token::<usize>();
    let k = scan.token::<usize>();
    let mut a = vec![0;n+1];
    let mut b = vec![0;m+1];
    for i in 0..n {
        let w = scan.token::<usize>();
        a[i+1] = a[i] +w;
    }
    for i in 0..m {
        let w = scan.token::<usize>();
        b[i+1] = b[i] +w;
    }
    logln!("{:?}", a);
    logln!("{:?}", b);
    let f = |x| {
        for i in 0..=x {
            logln!("{}",x);
            let ai = i;
            let bi = x-i;
            if ai > n { continue; }
            if bi > m { continue; }
            if k >= a[ai] + b[bi] {
                logln!("{},{},{},{}", a[ai], b[bi], ai, bi);
                return true;
            }
        }
        return false;
    };
    let mut l = 0;
    let mut r = n+m+1;
    while l+1 < r {
        let x = (l+r)/2;
        if f(x) { l=x; } else { r=x; }
    }
    writeln!(out, "{}", l).ok();
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
mod abc172c {
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
3 4 240
60 90 120
80 150 80 150
" , "\
3
");
    test_macro!(test2, b"\
3 4 730
60 90 120
80 150 80 150
" , "\
7
");
    test_macro!(test3, b"\
5 4 1
1000000000 1000000000 1000000000 1000000000 1000000000
1000000000 1000000000 1000000000 1000000000
" , "\
0
");

}
