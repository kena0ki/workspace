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
2 4
" , "\
6
");

    test_macro!(test2, b"\
1 1
" , "\
2
");

    test_macro!(test3, b"\
123456789000 123456789050
" , "\
2125824
");

}

// https://atcoder.jp/contests/abc195/tasks/abc195_e
fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let a = scan.token::<usize>();
    let b = scan.token::<usize>();
    let d = b-a;
    let mut primes = Vec::new();
    let mut sieve = vec![false;d+1];
    for i in 2..d+1 {
        if sieve[i] {
            continue;
        }
        primes.push(i);
        for j in (i..d+1).step_by(i) {
            sieve[j] = true;
        }
    }
    let mut masks = Vec::with_capacity(d+1);
    for i in 0..d+1 {
        let x = a+i;
        let mut mask = 0;
        for j in 0..primes.len() {
            if x%primes[j] == 0 {
                mask |= 1<<j;
            }
        }
        logln!("{},{:b}",x,mask);
        masks.push(mask);
    }
    logln!("{}",masks.len());
    let n2 = 1<<primes.len();
    let mlen = masks.len();
    let mut dp = vec![0usize;n2];
    dp[0] = 1;
    logln!("{}", d);
    for j in 0..mlen {
        for i in 0..n2 {
            let mask=masks[j];
            if (mask&i) == 0 {
                dp[i|mask] += dp[i];
            }
        }
        logln!("{:?}",dp);
    }
    let ans = dp.iter().sum::<usize>();
    writeln!(out, "{}a", ans).ok();
}

