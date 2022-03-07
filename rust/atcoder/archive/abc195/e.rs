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
35
AT
" , "\
Takahashi
");

    test_macro!(test2, b"\
5
12345
AAAAT
" , "\
Aoki
");

    test_macro!(test3, b"\
5
67890
TTTTA
" , "\
Takahashi
");

    test_macro!(test4, b"\
5
12345
ATATA
" , "\
Aoki
");

}

// https://atcoder.jp/contests/abc195/tasks/abc195_e
fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let mut vs = scan.token_bytes().iter().map(|v| (v-b'0') as usize).collect::<Vec<_>>();
    vs.reverse();
    let mut vx = scan.token_bytes();
    vx.reverse();
    let mut dp = vec![vec![false;7];n+1];
    dp[0][0] = true;
    let mut base=1;
    for i in 0..n {
        let s = vs[i]*base;
        base= (base*10)%7;
        for j in 0..7 {
            let pj = (j+s)%7;
            if vx[i] == b'T' {
                dp[i+1][j] = dp[i][pj] || dp[i][j];
            }
            if vx[i] == b'A' {
                dp[i+1][j] = dp[i][pj] && dp[i][j];
            }
        }
        logln!("{:?}",dp[i+1]);
    }
    let ans = if dp[n][0] { "Takahashi" } else { "Aoki" };
    writeln!(out, "{}", ans).ok();
}

