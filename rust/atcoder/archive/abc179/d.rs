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
5 2
1 1
3 4
" , "\
4
");

    test_macro!(test2, b"\
5 2
3 3
5 5
" , "\
0
");

    test_macro!(test3, b"\
5 1
1 2
" , "\
5
");

    test_macro!(test4, b"\
60 3
5 8
1 3
10 15
" , "\
221823067
");

}

// https://atcoder.jp/contests/abc179/tasks/abc179_d
fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let k = scan.token::<usize>();
    let mut vw = vec![(0,0);k];
    for i in 0..k {
        let l = scan.token::<usize>();
        let r = scan.token::<usize>();
        vw[i] = (l,r);
    }
    let mut dp = vec![0;n+1];
    let mut dps = vec![0;n+2];
    dp[1] = 1;
    let md = 998244353usize;
    for i in 1..n+1 {
        let mut cum = 0;
        for j in 0..k {
            let (l,r) = vw[j];
            if i<=l { continue; }
            let up = i-l+1;
            let low = i.saturating_sub(r);
            cum += dps[up] + md - dps[low];
            cum %= md;
            logln!("{},{},{},{},{}",i,l,r,up,low);
        }
        dp[i] += cum;
        dp[i] %= md;
        dps[i+1] += dps[i] + dp[i];
        dps[i+1] %= md;
    }
    logln!("{:?}",dp);
    logln!("{:?}",dps);
    writeln!(out, "{}", dp[n]).ok();
}

