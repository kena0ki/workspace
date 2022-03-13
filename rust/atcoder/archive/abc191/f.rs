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
3
6 9 12
" , "\
2
");

    test_macro!(test2, b"\
4
8 2 12 6
" , "\
1
");

    test_macro!(test3, b"\
7
30 28 33 49 27 37 48
" , "\
7
");

}

pub fn fast_gcd(mut a: usize, mut b: usize) -> usize {
    if a == 0 {
        return b;
    }
    while b != 0 {
        a %= b;
        std::mem::swap(&mut a, &mut b);
    }
    a
}

// https://atcoder.jp/contests/abc222/tasks/abc222_a
// TLE
fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let mut va = Vec::with_capacity(n);
    let mut amin = 1<<60;
    for _ in 0..n {
        let a = scan.token::<usize>();
        va.push(a);
        amin = amin.min(a);
    }
    let mut dp = HashSet::with_capacity(n);
    for i in 0..n {
        let a = va[i];
        dp.insert(a);
        let pre = dp.clone();
        for &j in &pre {
            dp.insert(fast_gcd(j,a));
        }
        logln!("{:?}",dp);
    }
    let mut ans = 0;
    for &i in &dp {
        if i <= amin {
            ans+=1;
        }
    }
    writeln!(out, "{}", ans).ok();
}

