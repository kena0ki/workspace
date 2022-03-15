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
99 99 99
" , "\
1.000000000
");

    test_macro!(test2, b"\
98 99 99
" , "\
1.331081081
");

    test_macro!(test3, b"\
0 0 1
" , "\
99.000000000
");

    test_macro!(test4, b"\
31 41 59
" , "\
91.835008202
");

}

// https://atcoder.jp/contests/abc184/tasks/abc184_d
fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let a = scan.token::<usize>();
    let b = scan.token::<usize>();
    let c = scan.token::<usize>();
    let max = 100;
    let mut dp = vec![vec![vec![0f64;max+1];max+1];max+1];
    for i in (a..max).rev() { for j in (b..max).rev() { for k in (c..max).rev() { 
        let fi = i as f64;
        let fj = j as f64;
        let fk = k as f64;
        let sum = fi+fj+fk;

        dp[i][j][k] = fi/sum * dp[i+1][j][k]
                     +fj/sum * dp[i][j+1][k]
                     +fk/sum * dp[i][j][k+1] + 1f64;
    }}}
    writeln!(out, "{}", dp[a][b][c]).ok();
}

