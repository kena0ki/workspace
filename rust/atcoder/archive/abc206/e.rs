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
3 7
" , "\
2
");

    test_macro!(test2, b"\
4 10
" , "\
12
");

    test_macro!(test3, b"\
1 1000000
" , "\
392047955148
");

}

// https://atcoder.jp/contests/abc206/tasks/abc206_e
fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let l = scan.token::<usize>();
    let r = scan.token::<usize>();
    let mut vg = vec![0;r+1];
    for i in (2..r+1).rev() {
        let lq = (l-1)/i;
        let rq = r/i;
        let q = rq-lq;
        vg[i] = q*(q-1);
        for j in (2*i..r+1).step_by(i) {
            vg[i] -= vg[j];
        }
    }
    logln!("{:?}",vg);
    for i in l.max(2)..r+1 {
        vg[i] -= ((r/i)-1)*2;
    }
    let ans = vg.iter().sum::<usize>();
    writeln!(out, "{}", ans).ok();
}

