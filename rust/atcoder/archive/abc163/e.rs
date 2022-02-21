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
mod abc163e {
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
4
1 3 4 2
" , "\
20
");

    test_macro!(test2, b"\
6
5 5 6 1 1 1
" , "\
58
");

    test_macro!(test3, b"\
6
8 6 9 1 2 1
" , "\
85
");

}

// https://atcoder.jp/contests/abc163/tasks/abc163_e
fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let mut va = Vec::with_capacity(n);
    for i in 0..n {
        let a = scan.token::<usize>();
        va.push((a,i));
    }
    va.sort_unstable_by(|a,b| b.cmp(a));
    let mut dp = vec![vec![0;n+1];n+1];
    for i in 0..n {
        let (a,ai) = va[i];
        logln!("{:?}", va[i]);
        let ni = i+1;
        for j in 0..i+1 {
            let nj = j+1;
            let l = j;
            if l <= ai {
                dp[ni][nj] = dp[ni][nj].max(dp[i][j] + (a * (ai-l)));
            }
            let r = n-1-(i-j);
            if  ai <= r {
                dp[ni][j] = dp[ni][j].max(dp[i][j] + (a * (r-ai)));
            }
        }
        logln!("{:?}",dp[ni]);
    }
    let ans = dp[n].iter().max().copied().unwrap();
    writeln!(out, "{}", ans).ok();
}

