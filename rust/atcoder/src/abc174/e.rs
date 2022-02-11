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

// https://atcoder.jp/contests/abc174/tasks/abc174_e
fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let k = scan.token::<usize>();
    let mut a = vec![0;n];
    let mut mx = 0;
    for i in 0..n {
        a[i] = scan.token::<usize>();
        mx = mx.max(a[i]);
    }
    let f = |x| {
        let mut tot=0;
        for i in 0..n {
            tot += ((a[i] + x-1)/x) -1;
        }
        return tot;
    };
    if f(1) <= k {
        writeln!(out, "1").ok();
        return;
    }
    let mut l = 1;
    let mut r = 1001001001;
    while l+1 < r {
        let x = (l+r)/2;
        if f(x) <= k { r = x; }
        else { l = x; }
    }
    writeln!(out, "{}", r).ok();
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
mod abc174c {
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
2 3
7 9
" , "\
4
");
    test_macro!(test2, b"\
3 0
3 4 5
" , "\
5
");
    test_macro!(test3, b"\
10 10
158260522 877914575 602436426 24979445 861648772 623690081 433933447 476190629 262703497 211047202
" , "\
292638192
");

}
