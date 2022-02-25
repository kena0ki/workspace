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
1 5
3 3
" , "\
3
");

    test_macro!(test2, b"\
3
6 5
-1 10
3 3
" , "\
5
");

    test_macro!(test3, b"\
4
7 10
-10 3
4 3
-4 3
" , "\
16
");

    test_macro!(test4, b"\
20
-8 1
26 4
0 5
9 1
19 4
22 20
28 27
11 8
-3 20
-25 17
10 4
-18 27
24 28
-11 19
2 27
-2 18
-1 12
-24 29
31 29
29 7
" , "\
110
");

}

fn _pow(val: usize, mut power: usize, modulus: usize) -> usize{
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

// https://atcoder.jp/contests/abc158/tasks/abc158_f
// WA
fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let mut vx = Vec::with_capacity(n);
    for _ in 0..n {
        let x = scan.token::<i64>();
        let d = scan.token::<i64>();
        vx.push((x,d));
    }
    let inf = 1<<50;
    vx.push((inf,0));
    vx.sort_unstable_by(|a,b| b.cmp(a));
    let f = |v:i64, vx: &Vec<(i64,i64)>| {
        let mut l = 0;
        let mut r = vx.len()-1;
        while l+1 < r {
            let m = (l+r)/2;
            if vx[m].0 < v{
                r = m;
            } else {
                l = m;
            }
        }
        return l;
    };
    const MOD:usize = 998244353;
    logln!("{:?}", vx);
    let mut dp = vec![0;n+1];
    let mut vk = vec![usize::max_value();n+1];
    dp[0] = 1;
    for i in 1..n+1 {
        let (x,d) = vx[i];
        let mut k = f(x+d,&vx);
        // TODO rmq
        vk[i] = k;
        logln!("{},{:?}", k,vk);
        dp[i] = (dp[i-1] + dp[vk[i]]) % MOD;
        logln!("{:?}", dp);
    }
    writeln!(out, "{}", dp[n]).ok();
}

