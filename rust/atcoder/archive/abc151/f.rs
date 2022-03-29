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
2
0 0
1 0
" , "\
0.500000000000000000
");

    test_macro!(test2, b"\
3
0 0
0 1
1 0
" , "\
0.707106781186497524
");

    test_macro!(test3, b"\
10
10 9
5 9
2 0
0 0
2 7
3 3
2 5
10 0
3 7
1 9
" , "\
6.726812023536805158
");

}

// https://atcoder.jp/contests/abc151/tasks/abc151_f
fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let mut vp = Vec::with_capacity(n);
    for _ in 0..n {
        let x = scan.token::<i64>();
        let y = scan.token::<i64>();
        vp.push((x,y));
    }
    let mut max = 0;

    for i in 0..n {
        let (x1,y1) = vp[i];
        for j in 0..n {
            let (x2,y2) = vp[j];
            let rd = (x1-x2)*(x1-x2) + (y1-y2)*(y1-y2);
            if max < rd {
                max = rd;
            }
        }
    }
    let ans = (max as f64).sqrt() / 2f64;

    writeln!(out, "{}", ans).ok();
}

