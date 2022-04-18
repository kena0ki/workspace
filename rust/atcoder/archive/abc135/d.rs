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
??2??5
" , "\
768
");

    test_macro!(test2, b"\
?44
" , "\
1
");

    test_macro!(test3, b"\
7?4
" , "\
0
");

    test_macro!(test4, b"\
?6?42???8??2??06243????9??3???7258??5??7???????774????4?1??17???9?5?70???76???
" , "\
153716888
");

}

// https://atcoder.jp/contests/abc135/tasks/abc135_d
fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    const MOD:usize = 1000000007;
    let mut vs = scan.token_bytes();
    vs.reverse();
    let mut dp = vec![0;13];
    dp[0] = 1;
    let mut base = 1;
    for i in 0..vs.len() {
        let s = vs[i];
        let mut nxt = vec![0;13];
        for j in 0..10 {
            if s != b'?' && s-b'0' != (j as u8) { continue; }
            let r = (j*base)%13;
            for k in 0..13 {
                let nk = (k+r)%13;
                nxt[nk] += dp[k];
                nxt[nk] %= MOD;
            }
        }
        dp = nxt;
        base = (base*10) % 13;
    }
    let ans = dp[5];
    writeln!(out, "{}", ans).ok();
}

