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
36
" , "\
8
");

    test_macro!(test2, b"\
314159265358979323846264338327950288419716939937551058209749445923078164062862089986280348253421170
" , "\
243
");

}

// https://atcoder.jp/contests/abc155/tasks/abc155_e
fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let mut vd = scan.token_bytes().iter()
        .map(|v| (v-b'0') as usize).collect::<Vec<_>>();
    vd.reverse();
    vd.push(0);
    let n = vd.len();
    let inf = 1<<60;
    let mut dp = vec![inf;2];
    dp[0] = 0;
    for i in 0..n {
        let d = vd[i];
        let pre = dp.clone();
        dp = vec![inf;2];
        for j in 0..2 {
            let d = d+j;
            if d == 0 {
                dp[0] = dp[0].min(pre[j]);
            } else if d == 10 {
                dp[1] = dp[1].min(pre[j]);
            } else{
                dp[0] = dp[0].min(pre[j]+d);
                dp[1] = dp[1].min(pre[j]+(10-d));
            }
        }
    }
    writeln!(out, "{}",dp[0]).ok();
}

