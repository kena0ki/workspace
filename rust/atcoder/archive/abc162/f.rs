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
mod abc162f {
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
6
1 2 3 4 5 6
" , "\
12
");

    test_macro!(test2, b"\
5
-1000 -100 -10 0 10
" , "\
0
");

    test_macro!(test3, b"\
5
-1000 -100 -10 0 10
" , "\
0
");

    test_macro!(test4, b"\
27
18 -28 18 28 -45 90 -45 23 -53 60 28 -74 -71 35 -26 -62 49 -77 57 24 -70 -93 69 -99 59 57 -49
" , "\
295
");

}

// https://atcoder.jp/contests/abc222/tasks/abc222_a
fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let inf = 1<<50;
    let mut dp = vec![vec![vec![-inf;2];3];n+1];
    let end = n%2 + 2;
    dp[0][0][0] = 0;
    for i in 0..n {
        let a = scan.token::<i64>();
        for j in 0..end {
            dp[i+1][j][1] = dp[i+1][j][1].max(dp[i][j][0]+a);
            dp[i+1][j][0] = dp[i+1][j][0].max(dp[i][j][1]);
            if j < 2 {
                dp[i+1][j+1][0] = dp[i+1][j+1][0].max(dp[i][j][0]);
            }
        }
        logln!("{:?}", dp[i+1]);
    }
    let mut ans = -inf;
    for j in (n%2)..end { for k in 0..2 {
        ans = ans.max(dp[n][j][k]);
    }}
    writeln!(out, "{}", ans).ok();
}

