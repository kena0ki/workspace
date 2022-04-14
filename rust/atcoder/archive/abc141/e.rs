// template

use std::io::{BufRead, BufWriter, Write};
#[allow(unused)]
use std::collections::*;

pub fn z_algorithm(text: &[impl Eq]) -> Vec<usize> {
    let n = text.len();
    let (mut l, mut r) = (1, 1);
    let mut z = Vec::with_capacity(n);
    z.push(n);
    for i in 1..n {
        if r > i + z[i - l] {
            z.push(z[i - l]);
        } else {
            l = i;
            while r < i || (r < n && text[r - i] == text[r]) {
                r += 1;
            }
            z.push(r - i);
        }
    }
    z
}

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
5
ababa
" , "\
2
");

    test_macro!(test2, b"\
2
xy
" , "\
0
");

    test_macro!(test3, b"\
13
strangeorange
" , "\
5
");

}

// https://atcoder.jp/contests/abc141/tasks/abc141_e
fn _solve_z(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let s = scan.token_bytes();
    let mut ans = 0;
    for i in 0..n-1 {
        let test = [&s[i..],b"$",&s[i..]].concat();
        let res = z_algorithm(&test);
        let m = n-i;
        let start = m+1;
        for j in start..start+m {
            let now = res[j].min(j-start);
            ans = ans.max(now);
        }
    }
    writeln!(out, "{}", ans).ok();
}

// https://atcoder.jp/contests/abc141/tasks/abc141_e
fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let s = scan.token_bytes();
    let mut dp = vec![vec![0;n];n];
    let mut ans = 0;
    for i in 0..n { for j in i..n {
        if s[i] == s[j] {
            if i>=1 {
                dp[i][j] = dp[i-1][j-1]+1;
            } else {
                dp[i][j] = 1;
            }
            let now = dp[i][j].min(j-i);
            ans = ans.max(now);
        }
    } }
    writeln!(out, "{}", ans).ok();
}

