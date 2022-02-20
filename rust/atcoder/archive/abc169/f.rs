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

const MOD: usize = 998244353;

/// Gets the power of this value.
pub fn pow(val:usize, mut power: usize) -> usize{
    let mut square = val;
    let mut ret = 1;
    while 0 < power {
        if (power & 1) == 1{
            ret *= square;
            ret %= MOD;
        }
        square *= square;
        square %= MOD;
        power >>= 1;
    }
    return ret;
}
/// Gets the inverse of this value.
pub fn inv(val:usize) -> usize {
    return pow(val, MOD - 2);
}

// https://atcoder.jp/contests/abc169/tasks/abc169_f
fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let s = scan.token::<usize>();
    let mut va = vec![0;n];
    for i in 0..n {
        va[i] = scan.token::<usize>();
    }
    let mut dp = vec![vec![0;s+1];n+1];
    //let itwo = inv(2);
    dp[0][0] = 1;
    for i in 0..n { for j in 0..s+1 {
        dp[i+1][j] = (dp[i+1][j] + (dp[i][j] * 2) % MOD) % MOD;
        if j + va[i] <= s {
            dp[i+1][j+va[i]] = (dp[i+1][j+va[i]] + (dp[i][j] * 1) % MOD) %MOD;
            //dp[i+1][j+va[i]] = (dp[i+1][j+va[i]] + (dp[i][j] * itwo) % MOD) %MOD;
        }
    } }

    //let ans = (dp[n][s] * pow(2, n)) % MOD;
    let ans = dp[n][s];
    writeln!(out, "{}",ans).ok();
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
mod abc169e {
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
3 4
2 2 4
" , "\
6
");

    test_macro!(test2, b"\
5 8
9 9 9 9 9
" , "\
0
");

    test_macro!(test3, b"\
10 10
3 1 4 1 5 9 2 6 5 3
" , "\
3296
");

}
