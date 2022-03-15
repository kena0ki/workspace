// template

use std::io::{BufRead, BufWriter, Write};
#[allow(unused)]
use std::collections::*;

use std::{ops::{Add, Index, IndexMut, Mul, Neg, Sub}, fmt::Debug};

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
    pub fn pow(&self, mut n: u64) -> Self {
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

#[derive(Clone, Debug, PartialEq)]
pub struct Affine {
    a: Matrix,
    b: Vec<i64>,
}

impl Affine {
    pub fn entity(dim: usize) -> Self {
        return Self { a: Matrix::one(dim), b: vec![0i64;dim]};
    }
    pub fn new(a: Matrix, b: Vec<i64>) -> Self {
        return Self { a, b };
    }
    pub fn compose(&self, g: &Self) -> Self {
        let dim = self.b.len();
        let mut a = Matrix::zero(dim,dim);
        for i in 0..dim { for j in 0..dim { for k in 0..dim {
            a[i][j] += g.a[i][k]*self.a[k][j];
        }}}
        let mut b = vec![0i64; dim];
        for i in 0..dim { for j in 0..dim {
            b[i] += g.a[i][j] * self.b[j];
        }}
        for i in 0..dim {
            b[i] += g.b[i];
        }
        return Self { a, b };
    }
    pub fn transform(&self, v: &Vec<i64>) -> Vec<i64> {
        let dim = self.b.len();
        let mut nv = vec![0i64; dim];
        for i in 0..dim { for j in 0..dim {
            nv[i] += self.a[i][j] * v[j];
        }}
        for i in 0..dim {
            nv[i] += self.b[i];
        }
        return nv;
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
1
1 2
4
1
3 3
2
4 2
5
0 1
1 1
2 1
3 1
4 1
" , "\
1 2
2 -1
4 -1
1 4
1 0
");

    test_macro!(test2, b"\
2
1000000000 0
0 1000000000
4
3 -1000000000
4 -1000000000
3 1000000000
4 1000000000
2
4 1
4 2
" , "\
5000000000 4000000000
4000000000 5000000000
");

}

// https://atcoder.jp/contests/abc189/tasks/abc189_e
fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let mut vp = vec![vec![0;2];n];
    for i in 0..n {
       vp[i][0] = scan.token::<i64>();
       vp[i][1] = scan.token::<i64>();
    }
    let m = scan.token::<usize>();
    let mut vo = vec![(0,0);m];
    for i in 0..m {
        let o = scan.token::<usize>();
        let mut p = 0;
        if o >= 3 {
            p = scan.token::<i64>();
        }
        vo[i] = (o,p);
    }
    let q = scan.token::<usize>();
    let mut va = vec![Vec::with_capacity(n);m+1];
    for i in 0..q {
        let a = scan.token::<usize>();
        let b = scan.token::<usize>()-1;
        va[a].push((i,b));
    }
    let mut af = Affine::entity(2);
    let mut ans = vec![(0,0);q];
    for i in 0..m+1 {
        for &(j,b) in &va[i] {
            let xy = af.transform(&vp[b]);
            ans[j]=(xy[0],xy[1]);
        }
        if i>=m { break; }
        let (o,p) = vo[i];
        if o == 1 {
            let mx = Matrix::from(vec![vec![0,1],vec![-1,0]]);
            let v = vec![0,0];
            let g = Affine::new(mx,v);
            af = af.compose(&g);
        } else if o == 2 {
            let mx = Matrix::from(vec![vec![0,-1],vec![1,0]]);
            let v = vec![0,0];
            let g = Affine::new(mx,v);
            af = af.compose(&g);
        } else if o == 3 {
            let mx = Matrix::from(vec![vec![-1,0],vec![0,1]]);
            let v = vec![2*p,0];
            let g = Affine::new(mx,v);
            af = af.compose(&g);
        } else if o == 4 {
            let mx = Matrix::from(vec![vec![1,0],vec![0,-1]]);
            let v = vec![0,2*p];
            let g = Affine::new(mx,v);
            af = af.compose(&g);
        }
    }
    for i in 0..q {
        let (x,y) = ans[i];
        writeln!(out, "{} {}",x,y).ok();
    }

}

