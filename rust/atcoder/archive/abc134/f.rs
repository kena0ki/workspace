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
3 2
" , "\
2
");

    test_macro!(test2, b"\
39 14
" , "\
74764168
");

}

// https://atcoder.jp/contests/abc134/tasks/abc134_f
fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    const MOD:usize = 1000000007;
    let n = scan.token::<usize>();
    let k = scan.token::<usize>();
    let mut dp = vec![vec![0;k+1];n];
    dp[0][0] = 1;
    for _ in 0..n {
        let mut xp = vec![vec![0;k+1];n];
        for j in 0..n { for l in 0..k+1 {
            let nj = j;
            let nl = l+2*nj;
            if nl <= k {
                xp[nj][nl] += (2*j+1)*dp[j][l];
                xp[nj][nl] %=MOD;
            }
            if j >= 1 {
                let nj = j-1;
                let nl = l+2*nj;
                if nl <= k {
                    xp[nj][nl] += j*j*dp[j][l];
                    xp[nj][nl] %=MOD;
                }
            }
            if j<=n-2 {
                let nj = j+1;
                let nl = l+2*nj;
                if nl <= k {
                    xp[nj][nl] += dp[j][l];
                    xp[nj][nl] %=MOD;
                }
            }
        } }
        dp = xp;
        logln!("{:?}",dp);
    }
    let ans = dp[0][k];
    writeln!(out, "{}", ans).ok();
}

