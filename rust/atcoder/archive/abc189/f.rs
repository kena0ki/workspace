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
2 2 0

" , "\
1.5000
");

    test_macro!(test2, b"\
2 2 1
1
" , "\
2.0000
");

    test_macro!(test3, b"\
100 6 10
11 12 13 14 15 16 17 18 19 20
" , "\
-1
");

    test_macro!(test4, b"\
100000 2 2
2997 92458
" , "\
201932.2222
");

}

#[derive(Clone,Copy,Default,Debug)]
struct X { a: f64, b: f64, }
impl X {
    pub fn new(a:f64,b:f64) -> Self {
        Self { a, b }
    }
    pub fn add(&self,rhs: Self) -> Self {
        Self { a:self.a+rhs.a, b:self.b+rhs.b }
    }
    pub fn sub(&self,rhs: Self) -> Self {
        Self { a:self.a-rhs.a, b:self.b-rhs.b }
    }
    pub fn div(&self,rhs: f64) -> Self {
        Self { a:self.a/rhs, b:self.b/rhs }
    }

}

// https://atcoder.jp/contests/abc189/tasks/abc189_f
fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let m = scan.token::<usize>();
    let k = scan.token::<usize>();
    let mut sa = HashSet::with_capacity(k);
    for _ in 0..k {
        let a = scan.token::<usize>();
        sa.insert(a);
    }
    let mut ve = vec![X::default();n+m];
    let mut sum = X::default();
    for i in (0..n).rev() {
        if sa.contains(&i) {
            ve[i] = X::new(1f64,0f64);
        } else {
            ve[i] = sum.div(m as f64).add(X::new(0f64,1f64));
        }
        sum = sum.sub(ve[i+m]).add(ve[i]);
    }
    if ve[0].a + 0.0000001 >= 1f64 {
        writeln!(out, "-1").ok();
    } else {
        let ans = ve[0].b/(1f64-ve[0].a);
        writeln!(out, "{}", ans).ok();
    }
}

