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
        let a = scan.token::<i64>();
        va.push((a,i as i64));
    }
    va.sort_unstable_by(|a,b| b.cmp(a));
    let inf = 1i64<<60;
    let mut dp = vec![vec![-inf;n+1];n+1];
    dp[0][0] = 0;
    for i in 0..n {
        let (a,pi) = va[i];
        logln!("{},{}", a,pi);
        for j in 0..i+1 {
            let l = j as i64;
            let r = (n-1-(i-j)) as i64;
            dp[i+1][j+1] = dp[i+1][j+1].max(dp[i][j] + (l-pi).abs() * a);
            dp[i+1][j] = dp[i+1][j].max(dp[i][j] + (r-pi).abs() * a);
        }
        logln!("{:?}", dp[i+1]);
    }
    let ans = dp[n].iter().max().unwrap();

    writeln!(out, "{}", ans).ok();
}

