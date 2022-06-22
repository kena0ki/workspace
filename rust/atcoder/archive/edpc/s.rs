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
3
" , "\
0
");

    test_macro!(test1, b"\
30
4
" , "\
6
");

    test_macro!(test2, b"\
1000000009
1
" , "\
2
");

    test_macro!(test3, b"\
98765432109876543210
58
" , "\
635270834
");

}

// https://atcoder.jp/contests/abc222/tasks/abc222_a
fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    const MOD:usize = 1000000007;
    let vb = scan.token_bytes().iter().map(|v| (v-b'0') as usize).collect::<Vec<_>>();
    let n = vb.len();
    let d = scan.token::<usize>();
    let mut dp = vec![vec![0;d];2];
    dp[0][0] = 1;
    for i in 0..n {
        let b = vb[i];
        let pre = dp.clone();
        dp = vec![vec![0;d];2];
        for less in 0..2 { for j in 0..d { for dg in 0..10 {
            if b < dg && less == 0 { continue; }
            let nless = if less == 1 || b > dg { 1 } else { 0 };
            let nj = (j+dg)%d;
            logln!("{},{}",j,dg);
            dp[nless][nj] += pre[less][j];
            dp[nless][nj] %= MOD;
        }}}
        logln!("{:?}",dp);
    }
    let mut ans = dp[0][0] + dp[1][0] + MOD - 1;
    ans %= MOD;
    writeln!(out, "{}", ans).ok();
}

