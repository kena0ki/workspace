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
2 10
10 1
1 1
" , "\
2
");

    test_macro!(test1, b"\
2 60
10 10
100 100
" , "\
110
");

    test_macro!(test2, b"\
3 60
10 10
10 20
10 30
" , "\
60
");

    test_macro!(test3, b"\
3 60
30 10
30 20
30 30
" , "\
50
");

    test_macro!(test4, b"\
10 100
15 23
20 18
13 17
24 12
18 29
19 27
23 21
18 20
27 15
22 25
" , "\
145
");

}

// https://atcoder.jp/contests/abc145/tasks/abc145_e
fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let t = scan.token::<usize>();
    let mut va = vec![(0,0);n];
    for i in 0..n {
        let a = scan.token::<usize>();
        let b = scan.token::<i64>();
        va[i] = (a,b);
    }
    va.sort_unstable();
    let mut dp = vec![0;t+1];
    for i in 0..n {
        let pre = dp.clone();
        for j in 0..t {
            let a = va[i].0;
            let b = va[i].1;
            let nj = (j+a).min(t);
            dp[nj] = dp[nj].max(pre[j]+b);
        }
    }

    let ans = dp.iter().max().copied().unwrap();
    writeln!(out, "{}", ans).ok();
}

