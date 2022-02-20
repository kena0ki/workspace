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

// https://atcoder.jp/contests/abc185/tasks/abc185_e
fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let m = scan.token::<usize>();
    let mut va = Vec::with_capacity(n);
    let mut vb = Vec::with_capacity(m);
    for _ in 0..n {
        let a = scan.token::<usize>();
        va.push(a);
    }
    for _ in 0..m {
        let b = scan.token::<usize>();
        vb.push(b);
    }
    let inf = 1001001001;
    let mut dp = vec![vec![inf;m+1];n+1];
    dp[0][0] = 0;
    for i in 0..n+1 {
        for j in 0..m+1 {
            if i >= 1 { dp[i][j] = dp[i][j].min(dp[i-1][j]+1) };
            if j >= 1 { dp[i][j] = dp[i][j].min(dp[i][j-1]+1) };
            if i >= 1 && j >= 1 {
                if va[i-1] == vb[j-1] {
                    dp[i][j] = dp[i][j].min(dp[i-1][j-1]);
                } else {
                    dp[i][j] = dp[i][j].min(dp[i-1][j-1]+1);
                }
            }
        }
        logln!("{:?}",dp);
    }
    let ans = dp[n][m];
    writeln!(out, "{}", ans).ok();
}

// https://atcoder.jp/contests/abc185/tasks/abc185_e
fn _solve_dist(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let m = scan.token::<usize>();
    let mut va = Vec::with_capacity(n);
    let mut vb = Vec::with_capacity(m);
    for _ in 0..n {
        let a = scan.token::<usize>();
        va.push(a);
    }
    for _ in 0..m {
        let b = scan.token::<usize>();
        vb.push(b);
    }
    let inf = 1001001001;
    let mut dp = vec![vec![inf;m+1];n+1];
    dp[0][0] = 0;
    for i in 0..n+1 {
        for j in 0..m+1 {
            if i < n { dp[i+1][j] = dp[i+1][j].min(dp[i][j]+1) };
            if j < m { dp[i][j+1] = dp[i][j+1].min(dp[i][j]+1) };
            if i < n && j < m {
                if va[i] == vb[j] {
                    dp[i+1][j+1] = dp[i+1][j+1].min(dp[i][j]);
                } else {
                    dp[i+1][j+1] = dp[i+1][j+1].min(dp[i][j]+1);
                }
            }
        }
        logln!("{:?}",dp);
    }
    // let mut ans = dp[n][0];
    // for j in 1..m+1 {
    //     ans = (ans+1).min(dp[n][j]);
    // }

    let ans = dp[n][m];
    writeln!(out, "{}", ans).ok();
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
mod abc185e {
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
4 3
1 2 1 3
1 3 1
" , "\
2
");

    test_macro!(test2, b"\
4 6
1 3 2 4
1 5 2 6 4 3
" , "\
3a
");

    test_macro!(test3, b"\
5 5
1 1 1 1 1
2 2 2 2 2
" , "\
5
");

}
