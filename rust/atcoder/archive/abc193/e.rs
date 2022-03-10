// template

use std::io::{BufRead, BufWriter, Write};
#[allow(unused)]
use std::collections::*;

pub fn crt(rm: &[i64], md: &[i64]) -> Option<(i64,i64)> {
    if rm.len() != md.len() {
        panic!("The size of remainders and moduli is not same.");
    }
    let mut r0 = 0;
    let mut m0 = 1;
    for (&(mut r1), &(mut m1)) in rm.iter().zip(md) {
        r1 = (r1+m1)%m1;
        if m1 < 1 {
            panic!("Modulus should be greater than 0, but input was {}", m1);
        }
        if m0 < m1 {
            std::mem::swap(&mut r0,&mut r1);
            std::mem::swap(&mut m0,&mut m1);
        }
        if m0%m1 == 0 {
            if r0%m1 != r1 {
                return None;
            }
            continue;
        }
        let (g,u,_v) = extended_gcd(m0,m1);
        if (r1-r0)%g != 0 {
            return None;
        }
        let m1g = m1/g;
        let im = (u+m1g)%m1g;
        let w = ((r1-r0)/g)%m1g;
        let x = r0 + ((w*im)%m1g)*m0;
        m0 *= m1g;
        r0 = (x+m0)%m0;
    }
    return Some((r0,m0));
}

fn extended_gcd(a: i64, b: i64) -> (i64, i64, i64) {
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
2
5 2 7 6
1 1 3 1
" , "\
20
infinity
");

    test_macro!(test1, b"\
3
5 2 7 6
1 1 3 1
999999999 1 1000000000 1
" , "\
20
infinity
1000000000999999999
");

}

// https://atcoder.jp/contests/abc193/tasks/abc193_f
fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let t = scan.token::<usize>();
    for _ in 0..t {
        let x = scan.token::<i64>();
        let y = scan.token::<i64>();
        let p = scan.token::<i64>();
        let q = scan.token::<i64>();
        let mut ans = i64::max_value();
        let m1 = 2*(x+y);
        let m2 = p+q;
        for i in x..(x+y) { for j in p..(p+q) {
            let x = crt(&vec![i,j], &vec![m1,m2]);
            if x.is_some() {
                ans = ans.min(x.unwrap().0);
            }
        } }
        if ans == i64::max_value() {
            writeln!(out, "infinity").ok();
        } else {
            writeln!(out, "{}", ans).ok();
        }
    }
}

