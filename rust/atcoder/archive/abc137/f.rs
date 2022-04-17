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
2
1 0
" , "\
1 1
");

    test_macro!(test2, b"\
3
0 0 0
" , "\
0 0 0
");

    test_macro!(test3, b"\
5
0 1 0 1 0
" , "\
0 2 0 1 3
");

}

pub fn lagrange_interp(vx: &Vec<i64>, vy: &Vec<i64>, modulus: usize) -> Vec<i64> {
    let n = vx.len();
    let md = modulus as i64;
    let vx = vx.iter().map(|x| (x%md+md)%md).collect::<Vec<_>>();
    let vy = vy.iter().map(|y| (y%md+md)%md).collect::<Vec<_>>();

    // (x - x_1) * (x - x_2) ... (x - x_n)
    // vprod[i] = coefficient of x^i
    let mut vprod = vec![0;n+1];
    vprod[0] = vx[0];
    vprod[1] = 1;
    for i in 1..n {
        let x = vx[i];
        let mut next = vec![0;n+1];
        for j in 0..i+1 {
            next[j] += md - (vprod[j]*x)%md;
            next[j] %= md;
            next[j+1] += vprod[j];
            next[j+1] %= md;
        }
        vprod = next;
    }

    // vconst[i] = y_i/(x_i - x_j) where j=0~n (j != i)
    let mut vconst = vec![0;n];
    for i in 0..n {
        let xi = vx[i];
        let mut co = 1;
        for j in 0..n {
            let xj = vx[j];
            if xi == xj { continue; }
            co *= (xi+md-xj)%md;
            co %= md;
        }
        co = vy[i]*invi(co,md);
        vconst[i] = co % md;
    }

    // Pre-calculation for Optimization of O(logN) time.
    let mut vinv = vec![0;n];
    for i in 0..n {
        let x = vx[i];
        if x == 0 { continue; }
        if x == md-1 {
            vinv[i] = 1;
            continue;
        }
        vinv[i] = invi(md-x, md);
    }

    // Coefficient of x^i
    let mut vcoef = vec![0;n];
    for i in 0..n {
        let x = vx[i];
        if x==0 {
            // a_n*x^n + a_n-1*x^n-1 ... a_1*x
            //  -> a_n*x^n-1 + a_n-1*x^n-2 ... a_1
            // vcoef[i] = a_i+1
            for j in 0..n {
                vcoef[j] += vprod[j+1]*vconst[i];
                vcoef[j] %= md;
            }
        } else {
            // a_n*x^n + a_n-1*x^n-1 ... a_1*x + a_0
            //  -> (b_n*x^n-1 + b_n-1*x^n-2 ... b_1)*(x - x_i)
            // vcoef[i] = b_i+1
            let mut co = 0;
            for j in 0..n {
                co = (vprod[j]+md-co)*vinv[i];
                //co = (vprod[j]+md-co)*invi(md-x,md);
                co %= md;
                vcoef[j] += co*vconst[i];
                vcoef[j] %= md;
            }
        }
    }
    return vcoef;
}

fn powi(val:i64, mut power: i64, modulus:i64) -> i64 {
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
fn invi(val: i64, modulus:i64) -> i64 {
    return powi(val, modulus - 2, modulus);
}

pub fn pow(val:usize, mut power: usize, modulus:usize) -> usize {
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
pub fn inv(val: usize, modulus:usize) -> usize {
    return pow(val, modulus - 2, modulus);
}

// https://atcoder.jp/contests/abc137/tasks/abc137_f
fn _solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let p = scan.token::<usize>();
    let mut va = Vec::with_capacity(p);
    let md = p;
    for _ in 0..p {
        let a = scan.token::<usize>();
        va.push(a);
    }
    let mut vx = vec![0;p+1];
    vx[1] = 1;
    for i in 1..p {
        let mut next = vec![0;p+1];
        for j in 0..i+1 {
            next[j] += md - (vx[j]*i)%md;
            next[j] %= md;
            next[j+1] += vx[j];
            next[j+1] %= md;
        }
        vx = next;
        logln!("{:?}",vx);
    }
    let mut vc = vec![0;p];
    for i in 0..p {
        let mut co = 1;
        for j in 0..p {
            if i == j { continue; }
            co *= (i+md-j)%md;
            co %= md;
        }
        co = va[i]*inv(co,md);
        vc[i] = co % md;
    }
    let mut vinv = vec![0;p];
    vinv[1] = 1;
    for i in 2..p {
        vinv[i] = inv(i, md);
    }
    let mut vb = vec![0;p];
    for i in 0..p {
        if i==0 {
            for j in 0..p {
                vb[j] += vx[j+1]*vc[i];
                vb[j] %= md;
            }
        } else {
            let mut t = 0;
            for j in 0..p {
                //t = (vx[j]+md-t)*vinv[(md-i)%md];
                t = (vx[j]+md-t)*inv(md-i,md);
                t %= md;
                vb[j] += t*vc[i];
                vb[j] %= md;
            }
        }
    }
    let mut vs = vec![" ";p];
    vs[p-1] = "\n";
    for i in 0..p {
        write!(out, "{}{}", vb[i],vs[i]).ok();
    }
}


// https://atcoder.jp/contests/abc137/tasks/abc137_f
fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let p = scan.token::<usize>();
    let mut vx = Vec::with_capacity(p);
    let mut vy = Vec::with_capacity(p);
    for i in 0..p {
        let a = scan.token::<i64>();
        vx.push(i as i64);
        vy.push(a);
    }
    let vans = lagrange_interp(&vx, &vy, p);
    let mut vs= vec![" ";p];
    vs[p-1]="\n";
    for i in 0..p {
        write!(out,"{}{}", vans[i],vs[i]).ok();
    }
}
