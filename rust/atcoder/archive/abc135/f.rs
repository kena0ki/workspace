// template

use std::io::{BufRead, BufWriter, Write};
#[allow(unused)]
use std::collections::*;

pub fn z_algorithm(text: &[impl Eq]) -> Vec<usize> {
    let n = text.len();
    let (mut l, mut r) = (1, 1);
    let mut z = Vec::with_capacity(n);
    z.push(n);
    for i in 1..n {
        if r > i + z[i - l] {
            z.push(z[i - l]);
        } else {
            l = i;
            while r < i || (r < n && text[r - i] == text[r]) {
                r += 1;
            }
            z.push(r - i);
        }
    }
    z
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
abcabab
ab
" , "\
3
");

    test_macro!(test2, b"\
aa
aaaaaaa
" , "\
-1
");

    test_macro!(test3, b"\
aba
baaab
" , "\
0
");

}

// https://atcoder.jp/contests/abc135/tasks/abc135_f
fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let f = |x:&Vec<u8>,y:&Vec<u8>| {
        let text = [x,&b"$"[..],y].concat();
        let vz = z_algorithm(&text);
        let xlen = x.len();
        let ylen = y.len();
        let mut dp = vec![0;ylen];
        for i in 0..ylen {
            let j = i+xlen+1;
            if vz[j] == xlen {
                if i>=xlen {
                    dp[i] += dp[i-xlen]+1;
                } else {
                    dp[i] = 1;
                }
            }
        }
        let res = dp.iter().max().copied().unwrap();
        return res;
    };

    let mut s = scan.token_bytes();
    let t = scan.token_bytes();
    s = [&s[..],&s[..]].concat();
    while s.len() < t.len()*3 {
        s = [&s[..],&s[..]].concat();
    }
    let res1 = f(&t,&s);
    let s = [&s[..],&s[..]].concat();
    let res2 = f(&t,&s);
    if res1 < res2 {
        writeln!(out, "-1").ok();
        return;
    }

    writeln!(out, "{}", res2).ok();
}

