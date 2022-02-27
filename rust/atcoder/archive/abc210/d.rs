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
3 4 2
1 7 7 9
9 6 3 7
7 8 6 4
" , "\
10
");

    test_macro!(test2, b"\
3 3 1000000000
1000000 1000000 1
1000000 1000000 1000000
1 1000000 1000000
" , "\
1001000001
");

}

// https://atcoder.jp/contests/abc210/tasks/abc210_d
fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let h = scan.token::<usize>();
    let w = scan.token::<usize>();
    let c = scan.token::<usize>();
    let mut va = vec![vec![0;w];h];
    for i in 0..h { for j in 0..w {
        let a = scan.token::<usize>();
        va[i][j] = a;
    }}
    let inf = 1<<50;
    let mut ans = inf;
    for _ in 0..2 {
        let mut dp = vec![vec![vec![inf;2];w];h];
        for i in 0..h { for j in 0..w { for k in 0..2 {
            dp[i][j][0] = dp[i][j][0].min(va[i][j]);
            if i >= 1 {
                dp[i][j][1] = dp[i][j][1].min(dp[i-1][j][k] + c);
            }
            if j >= 1 {
                dp[i][j][1] = dp[i][j][1].min(dp[i][j-1][k] + c);
            }
        }}
            logln!("{:?}", dp[i]);
        }
        for i in 0..h { for j in 0..w {
            ans = ans.min(dp[i][j][1]+va[i][j]);
            logln!("{},{},{}", i,j,ans);
        }}
        va.reverse();
    }
    writeln!(out, "{}", ans).ok();
}

