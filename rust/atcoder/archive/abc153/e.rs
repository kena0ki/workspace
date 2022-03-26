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
9 3
8 3
4 2
2 1
" , "\
4
");

    test_macro!(test2, b"\
100 6
1 1
2 3
3 9
4 27
5 81
6 243
" , "\
100
");

    test_macro!(test3, b"\
9999 10
540 7550
691 9680
700 9790
510 7150
415 5818
551 7712
587 8227
619 8671
588 8228
176 2461
" , "\
139815
");

}

// https://atcoder.jp/contests/abc153/tasks/abc153_e
fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let h = scan.token::<usize>();
    let n = scan.token::<usize>();
    let inf = 1<<60;
    let mut dp = vec![inf;h+1];
    dp[0] = 0;
    for _ in 0..n {
        let a = scan.token::<usize>();
        let b = scan.token::<usize>();
        for j in 0..h+1 {
            let nj = j+a;
            let nj = nj.min(h);
            dp[nj] = dp[nj].min(dp[j]+b);
        }
    }
    writeln!(out, "{}", dp[h]).ok();
}

