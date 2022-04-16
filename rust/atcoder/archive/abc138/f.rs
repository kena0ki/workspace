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
2 3
" , "\
3
");

    test_macro!(test2, b"\
10 100
" , "\
604
");

    test_macro!(test3, b"\
1 1000000000000000000
" , "\
68038601
");

}

pub fn pow(val:usize, mut power: usize, modulus:usize) -> usize {
    let mut square = val;
    let mut ret = 1;
    while 0 < power {
        if (power & 1) == 1{
            ret *= square;
            ret %= modulus;
        }
        square *= square;
        square %= modulus;
        power >>= 1;
    }
    return ret;
}
pub fn inv(val: usize, modulus:usize) -> usize {
    return pow(val, modulus - 2, modulus);
}

// https://atcoder.jp/contests/abc138/tasks/abc138_f
fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let l = scan.token::<usize>();
    let r = scan.token::<usize>();
    const MOD:usize = 1000000007;
    let n = 60;
    //let n = 6;
    let mut dp = vec![vec![vec![vec![0;2];2];2];n+1];
    dp[0][0][0][0] = 1;
    for i in 0..n { for j in 0..2 { for k in 0..2 { for s in 0..2 {
        for x in 0..2 { for y in 0..2 {
            if dp[i][j][k][s] == 0 { continue; }
            if x == 1 && y==0 { continue; }
            if s==0 && x==0 && y==1 { continue; }
            let t=n-1-i;
            if j==0 && x < (l>>t&1) { continue; }
            if k==0 && y > (r>>t&1) { continue; }
            let mut nj = j;
            if x > (l>>t&1) { nj = 1 }
            let mut nk = k;
            if y < (r>>t&1) { nk = 1 }
            let mut ns = s;
            if x==1 && y==1 { ns = 1 }
            dp[i+1][nj][nk][ns] += dp[i][j][k][s];
            dp[i+1][nj][nk][ns] %= MOD;
            //logln!("{},{},{},{},{},{},{:?}",i,j,k,s,x,y,dp[i+1]);
        }}
    }}}}
    let mut ans = 0;
    for j in 0..2 { for k in 0..2 {
        ans += dp[n][j][k][1];
        ans %= MOD;
    }}
    writeln!(out, "{}", ans).ok();
}

