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
3
BWB
WBW
BWB
" , "\
12
");

    test_macro!(test1, b"\
2
BB
BW
" , "\
2
");

    test_macro!(test2, b"\
3
BBB
BBB
W?W
" , "\
4
");

    test_macro!(test3, b"\
5
?????
?????
?????
?????
?????
" , "\
40
");

}

// https://atcoder.jp/contests/abc193/tasks/abc193_f
// WA
fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let mut vvc = Vec::with_capacity(n);
    for _ in 0..n {
        let vc = scan.token_bytes();
        vvc.push(vc);
    }
    let mut dp = vec![vec![vec![0usize;2];n];n];
    for i in 0..n { for j in 0..n {
        let c = vvc[i][j];
        for k in 0..2 { for pk1 in 0..2 { for pk2 in 0..2 {
            if k == 0 && c == b'B' { continue; }
            if k == 1 && c == b'W' { continue; }
            let mut p = 0;
            if i > 0 {
                let pc = vvc[i-1][j];
                if pk1 == 0 && pc == b'B' { continue; }
                if pk1 == 1 && pc == b'W' { continue; }
                let add = if k != pk1 { 1 } else { 0 };
                p += dp[i-1][j][pk1]+add;
            }
            if j > 0 {
                let pc = vvc[i][j-1];
                if pk2 == 0 && pc == b'B' { continue; }
                if pk2 == 1 && pc == b'W' { continue; }
                let add = if k != pk2 { 1 } else { 0 };
                logln!("{},{}",k,pk2);
                p += dp[i][j-1][pk2]+add;
            }
            if i > 0 && j > 0 {
                p -= dp[i-1][j-1][0].max(dp[i-1][j-1][1]);
            }
            dp[i][j][k] = dp[i][j][k].max(p);
        } } }
        logln!("{:?}",dp);
    } }
    let ans = dp[n-1][n-1][0].max(dp[n-1][n-1][1]);
    writeln!(out, "{}", ans).ok();
}

