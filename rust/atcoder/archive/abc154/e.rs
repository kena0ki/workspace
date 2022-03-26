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
100
1
" , "\
19
");

    test_macro!(test2, b"\
25
2
" , "\
14
");

    test_macro!(test3, b"\
314159
2
" , "\
937
");

    test_macro!(test4, b"\
9999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999
3
" , "\
117879300
");

}

// https://atcoder.jp/contests/abc154/tasks/abc154_e
fn _solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let vd = scan.token_bytes().iter().map(|v| (v-b'0') as usize).collect::<Vec<_>>();
    let k = scan.token::<usize>();
    let n = vd.len();
    let mut maxj = 0;
    let mut dp = vec![0;k+1];
    for i in 0..n {
        let d = vd[i];
        let pre = dp.clone();
        for j in 0..k {
            dp[j+1] += pre[j]*9;
        }
        if maxj < k {
            if d>=1 {
                dp[maxj+1] += d-1;
            }
        }
        if maxj <= k && d > 0 {
            dp[maxj] += 1;
        }
        if d > 0 {
            maxj += 1;
        }
        logln!("{:?}",dp);
    }
    let mut ans = dp[k];
    if maxj == k {
        ans+=1;
    }
    writeln!(out, "{}", ans).ok();
}

// https://atcoder.jp/contests/abc154/tasks/abc154_e
fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let vd = scan.token_bytes().iter().map(|v| (v-b'0') as usize).collect::<Vec<_>>();
    let k = scan.token::<usize>();
    let n = vd.len();
    let mut dp = vec![vec![0;k+1];2];
    dp[0][0] = 1;
    for i in 0..n {
        let d = vd[i];
        let pre = dp.clone();
        dp = vec![vec![0;k+1];2];
        for less in 0..2 { for j in 0..k+1 { for dg in 0..10 {
            if d < dg && less == 0 { continue; }
            if j==k && dg>0 { continue; }
            let nless = if less == 1 || dg < d { 1 } else { 0 };
            let nj = if dg > 0 { j+1 } else { j };
            dp[nless][nj] += pre[less][j];
        }}}
        logln!("{:?}",dp);
    }
    let ans = dp[0][k] + dp[1][k];
    writeln!(out, "{}", ans).ok();
}

