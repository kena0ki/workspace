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
1 1 2 2
" , "\
14
");

    test_macro!(test2, b"\
314 159 2653 589
" , "\
602215194
");

}
pub const fn pow(val:usize, mut power: usize, modulus:usize) -> usize {
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
pub const fn inv(val: usize, modulus:usize) -> usize {
    return pow(val, modulus - 2, modulus);
}

// https://atcoder.jp/contests/abc154/tasks/abc154_f
// WA
fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    const MOD:usize = 1000000007;
    let r1 = scan.token::<usize>();
    let c1 = scan.token::<usize>();
    let r2 = scan.token::<usize>();
    let c2 = scan.token::<usize>();
    let n = r2+c2;
    let mut fct = vec![0;n+1];
    fct[0] = 1;
    for i in 0..n {
        fct[i+1] = fct[i] * (i+1) % MOD;
    }
    let mut ifct = vec![0;n+1];
    ifct[n] = inv(fct[n],MOD);
    for i in (1..n+1).rev() {
        ifct[i-1] = ifct[i] * i % MOD;
    }
    let combi = |n,k| {
        return fct[n]*ifct[k]%MOD*ifct[n-k]%MOD;
    };
    let mut sum = 0;
    for i in c1..c2+1 {
        sum = sum + combi(r1+i,c1);
        sum %= MOD;
    }
    logln!("{}",sum);
    let mut ans = sum;
    for i in r1+1..r2+1 {
        let l = c1+i;
        let r = c2+i;
        let coe = (r+l)*(r-l+1)%MOD/2;
        //let coe = coe/i;
        let now = sum*coe/i%MOD;
        ans = (ans+now)%MOD;
        sum = now;
        logln!("{}",sum);
    }
    writeln!(out, "{}", ans).ok();
}

