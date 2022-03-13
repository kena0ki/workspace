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

    test_macro!(test0, b"\
3 20
3 6 8
" , "\
1
");

    test_macro!(test1, b"\
3 9999999999
3 6 8
" , "\
4999999994
");

    test_macro!(test2, b"\
1 1000000000000000000
1
" , "\
999999999999999999
");

}

// https://atcoder.jp/contests/abc192/tasks/abc192_f
fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let x = scan.token::<usize>();
    let mut va = vec![0;n];
    for i in 0..n {
        let a = scan.token::<usize>();
        va[i] = a;
    }
    let mut ans = usize::max_value();
    for k in 1..n+1 {
        let mut dp =vec![vec![None;n+2];n];
        dp[0][0]=Some(0);
        for i in 0..n {
            let a = va[i];
            let mut xdp = vec![vec![None;n+2];n];
            for j in 0..k { for l in 0..k+1 {
                if dp[j][l].is_none() { continue; };
                let now = dp[j][l].unwrap();
                xdp[j][l] = Some(xdp[j][l].unwrap_or(0).max(now));
                //let val = (dp[j][l].unwrap() * k) + j;
                //if val > x-a { continue; }
                //let next_val = val+a;
                //logln!("{},{},{},{}",k,j,l,next_val);
                //let nj = next_val % k;
                let nl = l+1;
                //xdp[nj][nl] = Some(xdp[nj][nl].unwrap_or(0).max(next_val/k));
                let nj = (j+a)%k;
                xdp[nj][nl] = Some(xdp[nj][nl].unwrap_or(0).max(now+a));
            }}
            dp = xdp;
            logln!("{:?}",dp);
        }
        let j = x%k;
        if dp[j][k].is_none() { continue; };
        //let val = dp[j][k].unwrap()*k + j;
        let val = dp[j][k].unwrap();
        ans = ans.min((x-val)/k);
    }
    writeln!(out, "{}",ans).ok();
}

