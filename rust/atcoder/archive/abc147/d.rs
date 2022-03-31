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
3
1 2 3
" , "\
6
");

    test_macro!(test2, b"\
10
3 1 4 1 5 9 2 6 5 3
" , "\
237
");

    test_macro!(test3, b"\
10
3 14 159 2653 58979 323846 2643383 27950288 419716939 9375105820
" , "\
103715602
");

}

// https://atcoder.jp/contests/abc147/tasks/abc147_d
fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    const MOD:usize = 1000000007;
    let n = scan.token::<usize>();
    let mut va = vec![0;n];
    for i in 0..n {
        let a = scan.token::<usize>();
        va[i] = a;
    }
    let mut ans = 0;
    let mut two = 1;
    for i in 0..60 {
        let mut ones = va[0]>>i&1;
        let mut sum = 0;
        for j in 1..n {
            let a = va[j]>>i&1;
            if a == 0 {
                sum+=ones;
            } else {
                sum+=j-ones;
            }
            ones+=a;
        }
        sum%=MOD;
        ans = (ans + sum*two%MOD)%MOD;
        two*=2;
        two%=MOD;
    }
    writeln!(out, "{}", ans).ok();
}

