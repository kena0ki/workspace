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
4 2
1 1 3 4
" , "\
11
");

    test_macro!(test2, b"\
6 3
10 10 10 -10 -10 -10
" , "\
360
");

    test_macro!(test3, b"\
3 1
1 1 1
" , "\
0
");

    test_macro!(test4, b"\
10 6
1000000000 1000000000 1000000000 1000000000 1000000000 0 0 0 0 0
" , "\
999998537
");

}

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

// https://atcoder.jp/contests/abc151/tasks/abc151_e
fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let k = scan.token::<usize>();
    let mut va = vec![0;n];
    for i in 0..n {
        let a = scan.token::<i64>();
        va[i] = a;
    }
    va.sort_unstable();
    const MOD: i64 = 1000000007;
    let mut fct = vec![0;n+1];
    fct[0] = 1;
    for i in 0..n {
        fct[i+1] = fct[i]* (i+1) as i64 %MOD;
    }
    let mut ifct = vec![0;n+1];
    ifct[n] = inv(fct[n], MOD);
    for i in (1..n+1).rev() {
        ifct[i-1] = ifct[i]* i as i64 %MOD;
    }
    let combi = |n,k| {
        return fct[n]*ifct[k]%MOD*ifct[n-k]%MOD;
    };
    let mut ans = 0;
    for i in 0..n {
        let a = va[i];
        if i+1 >= k {
            let max = a*combi(i,k-1)%MOD;
            ans = (ans+max)%MOD;
        }
        if n-i >= k {
            let min = a*combi(n-i-1,k-1)%MOD;
            ans = (ans+MOD-min)%MOD;
        }
    }
    writeln!(out, "{}",ans).ok();

}

