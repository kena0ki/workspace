// template

use std::io::{BufRead, BufWriter, Write};
#[allow(unused)]
use std::collections::*;

pub fn fast_gcd(mut a: i64, mut b: i64) -> i64 {
    if a == 0 {
        return b.abs();
    }
    while b != 0 {
        a %= b;
        std::mem::swap(&mut a, &mut b);
    }
    a.abs()
}
pub fn extended_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    if b == 0 {
        (a.abs(), a.signum(), 0)
    } else {
        let (d, coef_b, coef_a) = extended_gcd(b, a % b);
        (d, coef_a, coef_b - coef_a * (a / b))
    }
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

    test_macro!(test0, b"\
1
5 2 4
" , "\
2
");

    test_macro!(test1, b"\
4
10 4 3
1000 11 2
998244353 897581057 595591169
10000 6 14
" , "\
2
-1
249561088
3571
");

}

// https://atcoder.jp/contests/abc222/tasks/abc222_a
fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let t = scan.token::<i64>();
    for _ in 0..t {
        let mut n = scan.token::<i64>();
        let mut s = scan.token::<i64>();
        let mut k = scan.token::<i64>();
        let g0 = fast_gcd(n,k);
        if s%g0 == 0 {
            n /= g0;
            s /= g0;
            k /= g0;
        }
        let t = n-s;
        if (k%n) != 0 && t%(k%n) == 0 {
            let ans = t/(k%n);
            writeln!(out, "{}", ans).ok();
            continue;
        }
        let (g,a,b) = extended_gcd(n, k);
        logln!("{},{},{}", g,a,b);
        if b%g != 0 || a%g != 0 {
            writeln!(out, "-1").ok();
            continue;
        }
        let ik = (b/g%n+n)%n;
        let ans = (t*ik)%n;
        writeln!(out, "{}",ans).ok();
    }
}

