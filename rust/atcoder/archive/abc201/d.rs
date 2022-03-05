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
3 3
---
+-+
+--
" , "\
Takahashi
");

    test_macro!(test2, b"\
2 4
+++-
-+-+
" , "\
Aoki
");

    test_macro!(test3, b"\
1 1
-
" , "\
Draw
");

}

// https://atcoder.jp/contests/abc222/tasks/abc222_a
fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let h = scan.token::<usize>();
    let w = scan.token::<usize>();
    let inf = 1i64<<60;
    let mut dp = vec![vec![-inf;w];h];
    let mut vvb = Vec::with_capacity(h);
    for _ in 0..h {
        let vb = scan.token_bytes();
        let vb = vb.iter()
            .map(|&v| if v == b'+' { 1 } else { -1 }).collect::<Vec<_>>();
        vvb.push(vb);
    }
    for i in (0..h).rev() {
        for j in (0..w).rev() {
            let mut sub = -inf;
            if i == h-1 && j == w-1 { sub = 0; }
            if i+1 < h { sub = sub.max(dp[i+1][j]) }
            if j+1 < w { sub = sub.max(dp[i][j+1]) }
            dp[i][j] = dp[i][j].max(vvb[i][j] - sub);
        }
        logln!("{:?}", dp);
    }
    let d0 = vvb[0][0];
    let ans = if dp[0][0] > d0 {
        "Aoki"
    } else if dp[0][0] < d0 {
        "Takahashi"
    } else {
        "Draw"
    };
    writeln!(out, "{}", ans).ok();
}

