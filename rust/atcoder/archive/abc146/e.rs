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
5 4
1 4 2 3 5
" , "\
4
");

    test_macro!(test2, b"\
8 4
4 2 4 2 4 2 4 2
" , "\
7
");

    test_macro!(test3, b"\
10 7
14 15 92 65 35 89 79 32 38 46
" , "\
8
");

}

// https://atcoder.jp/contests/abc146/tasks/abc146_e
fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let k = scan.token::<usize>();
    let mut va = vec![0;n];
    for i in 0..n {
        let a = scan.token::<usize>();
        va[i] = a;
    }
    let mut vs = vec![0;n+1];
    for i in 0..n {
        vs[i+1] = va[i] + vs[i];
    }
    logln!("{:?}",vs);
    let mut mp = HashMap::<usize,usize>::with_capacity(n);
    let mut ans = 0;
    for i in 0..n+1 {
        if i>= k {
            let j = i-k;
            let s = (vs[j]-j)%k;
            *mp.entry(s).or_default() -= 1;
        }
        let s = (vs[i]-i)%k;
        let cnt =  mp.entry(s).or_default();
        ans += *cnt;
        *cnt += 1;
        logln!("{:?}",mp);
        logln!("{}",ans);
    }
    writeln!(out, "{}", ans).ok();
}

