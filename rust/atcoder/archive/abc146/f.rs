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
9 3
0001000100
" , "\
1 3 2 3
");

    test_macro!(test3, b"\
5 4
011110
" , "\
-1
");

    test_macro!(test2, b"\
6 6
0101010
" , "\
6
");


}

// https://atcoder.jp/contests/abc222/tasks/abc222_a
fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let m = scan.token::<usize>();
    let mut vs = scan.token_bytes();
    vs.reverse();
    let mut ans = Vec::with_capacity(n);
    let mut i = 0;
    while i < n {
        let end = (i+m+1).min(n+1);
        let mut next = 0;
        for j in (i+1..end).rev() {
            if vs[j] == b'0' {
                next=j;
                break;
            }
        }
        if next == 0 {
            writeln!(out, "-1").ok();
            return;
        }
        ans.push(next-i);
        i=next;
    }
    ans.reverse();

    let mut sp = vec![" ";ans.len()];
    sp[ans.len()-1] = "\n";
    for i in 0..ans.len() {
        write!(out, "{}{}", ans[i],sp[i]).ok();
    }
}

