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
2 3
10 1
1
15 1
2
30 2
1 2
" , "\
25
");

    test_macro!(test2, b"\
12 1
100000 1
2
" , "\
-1
");

    test_macro!(test3, b"\
4 6
67786 3
1 3 4
3497 1
2
44908 3
2 3 4
2156 3
2 3 4
26230 1
2
86918 1
3
" , "\
69942
");

}

// https://atcoder.jp/contests/abc142/tasks/abc142_e
fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let m = scan.token::<usize>();
    let mut va = vec![0;m];
    let mut vc = vec![0;m];
    for i in 0..m {
        let a = scan.token::<usize>();
        va[i] = a;
        let b = scan.token::<usize>();
        for _ in 0..b {
            let c = scan.token::<usize>()-1;
            vc[i] |= 1<<c;
        }
    }
    let n2 = 1<<n;
    let inf = 1<<60;
    let mut dp = vec![inf;n2];
    dp[0] = 0;
    for i in 0..m {
        let pre = dp.clone();
        for j in 0..n2 {
            let nj = j | vc[i];
            dp[nj] = dp[nj].min(pre[j] + va[i]);
        }
    }
    if dp[n2-1] == inf {
        writeln!(out, "{}", -1).ok();
    } else {
        writeln!(out, "{}", dp[n2-1]).ok();
    }
}

