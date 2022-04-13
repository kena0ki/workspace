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
2 1 2
" , "\
3
1
0
");

    test_macro!(test2, b"\
5
1 2 3 4 5
" , "\
5
2
1
1
1
");

    test_macro!(test3, b"\
4
1 3 3 3
" , "\
4
1
0
0
");

}

// https://atcoder.jp/contests/abc143/tasks/abc143_f
fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let mut vcnt = vec![(0,0);n];
    for _ in 0..n {
        let a = scan.token::<usize>()-1;
        vcnt[a] = (vcnt[a].0+1, a);
    }
    vcnt.sort_unstable();
    let mut s = 0;
    let mut ans = vec![0;n+1];
    for i in 1..n+1 {
        let now = vcnt.binary_search(&(i,0)).map_or_else(|v| v,|v| v);
        s+=n-now;
        for j in 1..(n/i)+1 {
            if s >= j*i {
                ans[j] = i;
            }
        }
    }
    for i in 1..n+1 {
        writeln!(out, "{}", ans[i]).ok();
    }
}

