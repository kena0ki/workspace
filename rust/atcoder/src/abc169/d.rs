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

// https://atcoder.jp/contests/abc169/tasks/abc169_d
fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let big = 1000000;
    let n = scan.token::<usize>();
    let mut primes = Vec::with_capacity(big+1);
    let mut sieve = vec![0;big+1];
    for i in 2..big+1 {
        if sieve[i] > 0 {
            continue;
        }
        primes.push(i);
        sieve[i] = i;
        for j in (i*i..big+1).step_by(i) {
            if sieve[j] == 0 {
                sieve[j] = i;
            }
        }
    }
    logln!("{}", primes.len());
    let mut ans = 0;
    let mut x = n;
    for p in primes {
        let mut i = 0;
        while x % p == 0 {
            for _ in 0..i {
                if x % p == 0 {
                    x /= p;
                }
            }
            if x % p == 0 {
                ans += 1;
                x /= p;
            }
            i += 1;
        }
    }
    if x > 1 {
        ans += 1;
    }
    writeln!(out, "{}", ans).ok();
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
mod abc169d {
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
24
" , "\
3
");

    test_macro!(test2, b"\
1
" , "\
0
");

    test_macro!(test3, b"\
64
" , "\
3
");

    test_macro!(test4, b"\
1000000007
" , "\
1
");

    test_macro!(test5, b"\
997764507000
" , "\
7
");

}
