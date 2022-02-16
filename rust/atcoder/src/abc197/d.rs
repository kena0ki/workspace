// template

use std::{io::{BufRead, BufWriter, Write}, f64::consts::PI};
#[allow(unused)]
use std::collections::*;

fn main() {
    let sin = std::io::stdin();
    let scan = &mut Scanner::new(sin.lock());
    let sout = std::io::stdout();
    let out = &mut BufWriter::new(sout.lock());
    solve(scan, out);
}

// https://atcoder.jp/contests/abc197/tasks/abc197_d
fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<f64>();
    let x1 = scan.token::<f64>();
    let y1 = scan.token::<f64>();
    let x2 = scan.token::<f64>();
    let y2 = scan.token::<f64>();
    let p1 = Complex::new(x1,y1);
    let p2 = Complex::new(x2,y2);
    let p3 = (p1+p2)/Complex::new(2f64,0f64);
    logln!("{:?}", p3);
    let r = PI*2f64/n;
    let v = Complex::new(f64::cos(r),f64::sin(r));
    let ans = p3 + ((p1 - p3) * v);

    writeln!(out, "{} {}", ans.real, ans.imag).ok();
}

use std::{ops::{Add, Div, Mul, Neg, Sub}, fmt::Debug};

/// Represents a complex number using floating-point arithmetic
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Complex {
    pub real: f64,
    pub imag: f64,
}
impl Complex {
    pub fn new(real: f64, imag: f64) -> Self {
        Self { real, imag }
    }
    pub fn from_polar(r: f64, th: f64) -> Self {
        Self::new(r * th.cos(), r * th.sin())
    }
    pub fn abs_square(self) -> f64 {
        self.real * self.real + self.imag * self.imag
    }
    pub fn argument(self) -> f64 {
        self.imag.atan2(self.real)
    }
    pub fn conjugate(self) -> Self {
        Self::new(self.real, -self.imag)
    }
    pub fn recip(self) -> Self {
        let denom = self.abs_square();
        Self::new(self.real / denom, -self.imag / denom)
    }
}
impl From<f64> for Complex {
    fn from(real: f64) -> Self {
        Self::new(real, 0.0)
    }
}
impl Neg for Complex {
    type Output = Self;
    fn neg(self) -> Self {
        Self::new(-self.real, -self.imag)
    }
}
impl Add for Complex {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self::new(self.real + other.real, self.imag + other.imag)
    }
}
impl Sub for Complex {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self::new(self.real - other.real, self.imag - other.imag)
    }
}
impl Mul for Complex {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        let real = self.real * other.real - self.imag * other.imag;
        let imag = self.imag * other.real + self.real * other.imag;
        Self::new(real, imag)
    }
}
#[allow(clippy::suspicious_arithmetic_impl)]
impl Div for Complex {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        self * other.recip()
    }
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
mod abc197d {
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
1 1
2 2
" , "\
2.00000000000 1.00000000000
");

    test_macro!(test2, b"\
6
5 3
7 4
" , "\
5.93301270189 2.38397459622
");

}
