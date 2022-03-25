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
3 2
" , "\
10
");

    test_macro!(test2, b"\
200000 1000000000
" , "\
607923868
");

    test_macro!(test3, b"\
15 6
" , "\
22583772
");

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
// https://atcoder.jp/contests/abc156/tasks/abc156_e
fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    const MOD:usize = 1000000007;
    let n = scan.token::<usize>();
    let m = scan.token::<usize>();
    let m = m.min(n-1);
    let mut fct = vec![0;n+1];
    fct[0] = 1;
    for i in 0..n {
        fct[i+1] = fct[i]*(i+1)%MOD;
    }
    let mut ifct = vec![0;n+1];
    ifct[n] = inv(fct[n], MOD);
    for i in (1..n+1).rev() {
        ifct[i-1] = ifct[i]*i%MOD;
    }
    let combi = |n,k| {
        return fct[n]*ifct[k]%MOD*ifct[n-k]%MOD;
    };
    let mut ans = 1;
    for i in 1..m+1 {
        let now = combi(n,i)*combi(n-1,n-i-1)%MOD;
        ans = ans+now;
        ans = ans%MOD;
    }
    writeln!(out, "{}", ans).ok();
}

