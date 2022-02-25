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
2 5
53
" , "\
6
");

    test_macro!(test1, b"\
4 3
3543
" , "\
6
");

    test_macro!(test2, b"\
4 2
2020
" , "\
10
");

    test_macro!(test3, b"\
20 11
33883322005544116655
" , "\
68
");

}

// https://atcoder.jp/contests/abc158/tasks/abc158_e
fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let p = scan.token::<usize>();
    let mut s = scan.token_bytes();
    let mut vs = vec![0;n+1];
    s.reverse();
    let mut ans = 0;
    if p == 2 || p == 5 {
        for i in 0..n {
            if (s[i] - b'0') as usize % p == 0 {
                ans += n-i;
            }
        }
    } else {
        let mut cnt = vec![0;p];
        cnt[0] = 1;
        let mut x = 1;
        for i in 0..n {
            let c = (vs[i] + (s[i]-b'0')as usize * x) % p;
            ans += cnt[c];
            cnt[c] += 1;
            vs[i+1] = c;
            x = (x*10) % p;
            logln!("{:?}",vs);
        }
    }
    writeln!(out, "{}", ans).ok();
}

