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
2 2 1 2
" , "\
4
");

    test_macro!(test2, b"\
3 3 4 1
" , "\
18
");

    test_macro!(test3, b"\
4 4 8 0
" , "\
36
");

}

// https://atcoder.jp/contests/abc196/tasks/abc196_d
fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let h = scan.token::<usize>();
    let w = scan.token::<usize>();
    let a = scan.token::<usize>();
    let b = scan.token::<usize>();
    let mut done = vec![0usize;w*h];
    let ans = f(h,w,a,b,0, &mut done);
    writeln!(out, "{}",ans).ok();
    fn f(h:usize, w:usize, a:usize, b:usize, i:usize,
        done: &mut Vec<usize>) -> usize {
        if i >= h*w {
            logln!("{:?}",done);
            return 1;
        }
        if done[i] > 0 {
            return f(h,w,a,b,i+1,done);
        }
        let mut res = 0;
        if a > 0 && i/w + 2 <= h && done[i+w] == 0 {
            done[i] = 2;
            done[i+w] = 2;
            res += f(h,w,a-1,b,i+1,done);
            done[i] = 0;
            done[i+w] = 0;
        }
        if a > 0 && i%w + 2 <= w && done[i+1] == 0 {
            done[i] = 1;
            done[i+1] = 1;
            res += f(h,w,a-1,b,i+1,done);
            done[i] = 0;
            done[i+1] = 0;
        }
        if b > 0 {
            done[i] = 3;
            res += f(h,w,a,b-1,i+1,done);
            done[i] = 0;
        }
        return res;
    }
}

