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
2
5 3
" , "\
1
");

    test_macro!(test2, b"\
2
3 5
" , "\
-1
");

    test_macro!(test3, b"\
3
1 1 2
" , "\
-1
");

    test_macro!(test4, b"\
8
10 9 8 7 6 5 4 3
" , "\
3
");

    test_macro!(test5, b"\
3
4294967297 8589934593 12884901890
" , "\
1
");

}

// https://atcoder.jp/contests/abc172/tasks/abc172_f
fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let a = scan.token::<usize>();
    let b = scan.token::<usize>();
    let mut c = 0;
    for _ in 0..n-2 {
        let d = scan.token::<usize>();
        c ^= d;
    }
    let inf = 1usize<<60;
//let inf = 1usize<<10;
    let mut dp = vec![vec![vec![inf;2];2];2];
    dp[0][0][0] = 0;
    let n2 = 45;
//let n2 = 8;
    for i in 0..n2 {
        let ai = a>>i&1;
        let bi = b>>i&1;
        let ci = c>>i&1;
        let pre = dp.clone();
        dp = vec![vec![vec![inf;2];2];2];
        for nj in 0..2 { for j in 0..2 { for k in 0..2 { for l in 0..2 {
            let nk = ai<k;
            let ai = ai^k; //ai-k
            let nk = nk || ai<nj;
            let ai = ai^nj; //ai-nj
            let nl = bi&l==1;
            let bi = bi^l; //bi+l
            let nl = nl || bi&nj==1;
            let bi = bi^nj; //bi+nj
            if ai ^ bi != ci { continue; }
            let nk = nk as usize;
            let nl = nl as usize;
            dp[nj][nk][nl] = dp[nj][nk][nl].min(pre[j][k][l] + (nj<<i));
            //logln!("i: {},{},{}",i,nj<<i,pre[j][k][l]);
        }}}}
        logln!("{:?}",dp);
    }
    let ans = dp[0][0][0];
    if ans == a || ans == inf {
        writeln!(out, "-1").ok();
    } else {
        writeln!(out, "{}", ans).ok();
    }
}

