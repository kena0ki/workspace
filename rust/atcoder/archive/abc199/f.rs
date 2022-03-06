// template

use std::io::{BufRead, BufWriter, Write};
#[allow(unused)]
use std::collections::*;

use std::{ops::{Add, Index, IndexMut, Mul, Neg, Sub}, fmt::Debug};

pub fn pow(val:i64, mut power: i64, modulus:i64) -> i64 {
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
pub fn inv(val: i64, modulus:i64) -> i64 {
    return pow(val, modulus - 2, modulus);
}

const MOD: i64 = 1000000007;

#[derive(Clone, PartialEq, Debug)]
pub struct Matrix {
    cols: usize,
    inner: Box<[i64]>,
}
impl Matrix {
    pub fn zero(rows: usize, cols: usize) -> Self {
        let inner = vec![0; rows * cols].into_boxed_slice();
        Self { cols, inner }
    }
    pub fn one(cols: usize) -> Self {
        let mut matrix = Self::zero(cols, cols);
        for i in 0..cols {
            matrix[i][i] = 1;
        }
        matrix
    }
    pub fn vector(vec: &[i64], as_row: bool) -> Self {
        let cols = if as_row { vec.len() } else { 1 };
        let inner = vec.to_vec().into_boxed_slice();
        Self { cols, inner }
    }
    pub fn pow(&self, mut n: i64) -> Self {
        let mut base = self.clone();
        let mut result = Self::one(self.cols);
        while n > 0 {
            if n % 2 == 1 {
                result = &result * &base;
            }
            base = &base * &base;
            n /= 2;
        }
        result
    }
    pub fn rows(&self) -> usize {
        self.inner.len() / self.cols
    }
    pub fn transpose(&self) -> Self {
        let mut matrix = Matrix::zero(self.cols, self.rows());
        for i in 0..self.rows() {
            for j in 0..self.cols {
                matrix[j][i] = self[i][j];
            }
        }
        matrix
    }
    pub fn recip(&self) -> Self {
        unimplemented!();
    }
}
impl Index<usize> for Matrix {
    type Output = [i64];
    fn index(&self, row: usize) -> &Self::Output {
        let start = self.cols * row;
        &self.inner[start..start + self.cols]
    }
}
impl IndexMut<usize> for Matrix {
    fn index_mut(&mut self, row: usize) -> &mut Self::Output {
        let start = self.cols * row;
        &mut self.inner[start..start + self.cols]
    }
}
impl Neg for &Matrix {
    type Output = Matrix;
    fn neg(self) -> Matrix {
        let inner = self.inner.iter().map(|&v| -v).collect();
        Matrix {
            cols: self.cols,
            inner,
        }
    }
}
impl Add for &Matrix {
    type Output = Matrix;
    fn add(self, other: Self) -> Matrix {
        let self_iter = self.inner.iter();
        let inner = self_iter
            .zip(other.inner.iter())
            .map(|(&u, &v)| u + v)
            .collect();
        Matrix {
            cols: self.cols,
            inner,
        }
    }
}
impl Sub for &Matrix {
    type Output = Matrix;
    fn sub(self, other: Self) -> Matrix {
        let self_iter = self.inner.iter();
        let inner = self_iter
            .zip(other.inner.iter())
            .map(|(&u, &v)| u - v)
            .collect();
        Matrix {
            cols: self.cols,
            inner,
        }
    }
}
impl Mul<i64> for &Matrix {
    type Output = Matrix;
    fn mul(self, scalar: i64) -> Matrix {
        let inner = self.inner.iter().map(|&v| v * scalar).collect();
        Matrix {
            cols: self.cols,
            inner,
        }
    }
}
impl Mul for &Matrix {
    type Output = Matrix;
    fn mul(self, other: Self) -> Matrix {
        assert_eq!(self.cols, other.rows());
        let mut matrix = Matrix::zero(self.rows(), other.cols);
        for i in 0..self.rows() {
            for k in 0..self.cols {
                for j in 0..other.cols {
                    matrix[i][j] += self[i][k] * other[k][j];
                    matrix[i][j] %= MOD;
                }
            }
        }
        matrix
    }
}

impl From<Vec<Vec<i64>>> for Matrix {
    fn from(v: Vec<Vec<i64>>) -> Self {
        let row = v.len();
        let col = v[0].len();
        let mut m = Matrix::zero(v.len(), v[0].len());
        for i in 0..row { for j in 0..col {
            m[i][j] = v[i][j];
        }}
        return m;
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

    test_macro!(test1, b"\
3 2 1
3 1 5
1 2
1 3
" , "\
3
500000005
500000008
");

    test_macro!(test2, b"\
3 2 2
12 48 36
1 2
1 3
" , "\
750000036
36
250000031
");

    test_macro!(test3, b"\
4 5 1000
578 173 489 910
1 2
2 3
3 4
4 1
1 3
" , "\
201113830
45921509
67803140
685163678
");

}

// https://atcoder.jp/contests/abc199/tasks/abc199_e
fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let m = scan.token::<usize>();
    let k = scan.token::<i64>();
    let mut va = vec![0;n];
    for i in 0..n {
        let a = scan.token::<i64>();
        va[i] = a;
    }
    let mut vvx = vec![vec![0;n];n];
    let mut deg = vec![0;n];
    let minv2 = inv(2*m as i64, MOD);
    for _ in 0..m {
        let u = scan.token::<usize>()-1;
        let v = scan.token::<usize>()-1;
        vvx[u][v] = minv2;
        vvx[v][u] = minv2;
        deg[u] += 1;
        deg[v] += 1;
    }
    for i in 0..n {
        vvx[i][i] = minv2 * (2*m - deg[i]) as i64;
        vvx[i][i] %= MOD;
    }
    let mx = Matrix::from(vvx);
    let ma = Matrix::vector(&va,false);
    let ans = &mx.pow(k) * &ma;
    for i in 0..n {
        writeln!(out, "{}",ans[i][0]).ok();
    }
}

