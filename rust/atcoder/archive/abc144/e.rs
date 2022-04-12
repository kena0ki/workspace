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
3 5
4 2 1
2 3 1
" , "\
2
");

    test_macro!(test2, b"\
3 8
4 2 1
2 3 1
" , "\
0
");

    test_macro!(test3, b"\
11 14
3 1 4 1 5 9 2 6 5 3 5
8 9 7 9 3 2 3 8 4 6 2
" , "\
12
");

}

// https://atcoder.jp/contests/abc144/tasks/abc144_e
fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let k = scan.token::<usize>();
    let mut va = vec![0;n];
    let mut vf = vec![0;n];
    for i in 0..n {
        let a = scan.token::<i64>();
        va[i] = a;
    }
    for i in 0..n {
        let f = scan.token::<i64>();
        vf[i] = f;
    }
    va.sort_unstable();
    vf.sort_unstable_by(|a,b| b.cmp(a));
    let inf = 1001001001001;
    //let inf = 1001001;
    let mut l = -1;
    let mut r = inf;
    let f = |x:i64| {
        let mut kcnt=0;
        for i in 0..n {
            let c = va[i] - x/vf[i];
            let c = c.max(0);
            kcnt += c as usize;
            //logln!("{}", kcnt);
        }
        return kcnt<=k;
    };
    while l+1<r {
        let x = (l+r)/2;
        if f(x) { r=x; } else { l=x; }
    }
    writeln!(out, "{}", r).ok();
}

