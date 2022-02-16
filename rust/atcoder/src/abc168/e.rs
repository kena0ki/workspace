// template

use std::io::{BufRead, BufWriter, Write};
#[allow(unused)]
use std::collections::*;

// https://atcoder.jp/contests/abc168/tasks/abc168_e
fn _solve2(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let mut mp = HashMap::<(i64,i64),(usize,usize)>::new();
    let mut zerocnt = 0;
    for _ in 0..n {
        let mut a = scan.token::<i64>();
        let mut b = scan.token::<i64>();
        if a == 0 && b == 0 {
            zerocnt += 1;
            continue;
        }
        let g = fast_gcd(a,b);
        a = a/g; b = b/g;
        if b < 0 || b == 0 && a < 0 {
            a = -a; b = -b;
        }
        let rot90 = a <= 0;
        if rot90 {
            a = -a;
            std::mem::swap(&mut a, &mut b);
        }
        if rot90 {
            (*mp.entry((a,b)).or_default()).0 += 1;
        } else {
            (*mp.entry((a,b)).or_default()).1 += 1;
        }
    }
    let md = 1000000007;
    let mut ans = 1;
    for (&k,&v) in &mp {
        let mut x = 1;
        x += pow(md, 2, v.0) +md-1;
        x %=md;
        x += pow(md, 2, v.1) +md-1;
        x %=md;
        ans *= x;
        ans = (ans+md)%md;
        logln!("{},{:?}", ans, k);
    }
    logln!("{:?}",mp);
    ans += md-1;
    ans %= md;
    ans += zerocnt;
    ans %= md;
    writeln!(out, "{}", ans).ok();
}

// https://atcoder.jp/contests/abc168/tasks/abc168_e
fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let mut mp = HashMap::<Rational,usize>::new();
    let mut zerocnt = 0;
    for _ in 0..n {
        let mut a = scan.token::<i64>();
        let mut b = scan.token::<i64>();
        if a == 0 && b == 0 {
            zerocnt += 1;
            continue;
        } else if a == 0 {
            b = b.abs();
        } else if b == 0 {
            a = a.abs();
        }
        let r = Rational::new(a,b);
        *mp.entry(r).or_default() += 1;
    }
    let md = 1000000007;
    let mut ans = 1;
    let mut done = HashSet::new();
    for (&k,&v) in &mp {
        if done.contains(&k) { continue; }
        done.insert(k);
        let mut x = pow(md, 2, v);
        x %= md;
        let r = if k.den == 0 || k.num == 0 {
            Rational::new(k.den,k.num)
        } else {
            Rational::new(-k.den,k.num)
        };
        x += pow(md,2, mp.get(&r).copied().unwrap_or(0));
        x %= md;
        done.insert(r);
        ans *= x+md-1;
        ans %= md;
        logln!("{},{:?}", ans, k);
    }
    logln!("{:?}",mp);
    ans = (ans+md-1)%md;
    writeln!(out, "{}", (ans+zerocnt)%md).ok();
}

/// Gets the power of this value.
pub fn pow(modulus: usize, val:usize, mut power: usize) -> usize{
    let mut square = val;
    let mut ret = 1;
    while 0 < power {
        if (power & 1) == 1{
            ret *= square;
            ret %= modulus;
        }
        square *= square;
        square %= modulus;
        power >>= 1;
    }
    return ret;
}

fn main() {
    let sin = std::io::stdin();
    let scan = &mut Scanner::new(sin.lock());
    let sout = std::io::stdout();
    let out = &mut BufWriter::new(sout.lock());
    solve(scan, out);
}

use std::{ops::{Add, Div, Mul, Neg, Sub}, fmt::{Display, Debug}};

/// Fast iterative version of Euclid's GCD algorithm
pub fn fast_gcd(mut a: i64, mut b: i64) -> i64 {
    if a == 0 {
        return b.abs();
    };
    while b != 0 {
        a %= b;
        std::mem::swap(&mut a, &mut b);
    }
    a.abs()
}

/// Represents a fraction reduced to lowest terms
#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct Rational {
    pub num: i64,
    pub den: i64,
}
impl Rational {
    pub fn new(num: i64, den: i64) -> Self {
        if num == 0 && den == 0 {
            panic!("0/0 is illegal");
        }
        let sign = if den < 0 { -1 } else { 1 };
        let g = fast_gcd(num, den) * sign;
        Self {
            num: num / g,
            den: den / g,
        }
    }
    pub fn abs(self) -> Self {
        Self {
            num: self.num.abs(),
            den: self.den,
        }
    }
    pub fn recip(self) -> Self {
        let sign = if self.num < 0 { -1 } else { 1 };
        Self {
            num: self.den / sign,
            den: self.num / sign,
        }
    }
}
impl From<i64> for Rational {
    fn from(num: i64) -> Self {
        Self { num, den: 1 }
    }
}
impl Neg for Rational {
    type Output = Self;
    fn neg(self) -> Self {
        Self {
            num: -self.num,
            den: self.den,
        }
    }
}
#[allow(clippy::suspicious_arithmetic_impl)]
impl Add for Rational {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self::new(
            self.num * other.den + self.den * other.num,
            self.den * other.den,
        )
    }
}
#[allow(clippy::suspicious_arithmetic_impl)]
impl Sub for Rational {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self::new(
            self.num * other.den - self.den * other.num,
            self.den * other.den,
        )
    }
}
impl Mul for Rational {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Self::new(self.num * other.num, self.den * other.den)
    }
}
#[allow(clippy::suspicious_arithmetic_impl)]
impl Div for Rational {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        self * other.recip()
    }
}
impl Ord for Rational {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.den == 0 && other.den == 0 {
            return self.num.cmp(&other.num);
        }
        (self.num * other.den).cmp(&(self.den * other.num))
    }
}
impl PartialOrd for Rational {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Display for Rational {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "{}", self.num as f64 / self.den as f64);
    }
}

impl Debug for Rational {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "{}/{}", self.num, self.den);
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
mod abc168c {
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

    test_macro!(test00, b"\
2
1000000000000000000 -1000000000000000000
-1000000000000000000 1000000000000000000
" , "\
1a
");

    test_macro!(test0, b"\
5
1 2
-1 1
2 -1
0 1
-1 0
" , "\
9a
");

    test_macro!(test1, b"\
3
1 2
-1 1
2 -1
" , "\
5a
");

    test_macro!(test2, b"\
10
3 2
3 2
-1 1
2 -1
-3 -9
-8 12
7 7
8 1
8 2
8 4
" , "\
479
");

}
