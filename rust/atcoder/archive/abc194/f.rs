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
                _solve(scan, output);
                solve(scan, output);
                assert_eq!($expected, std::str::from_utf8(output).unwrap());
            }
        };
    }

    test_macro!(test0, b"\
FF0FF 1
FF0FF 1
" , "\
b
");

    test_macro!(test1, b"\
10 1
" , "\
15
");

    test_macro!(test2, b"\
FF 2
" , "\
225
");

    test_macro!(test3, b"\
100 2
" , "\
226
");

    test_macro!(test4, b"\
1A8FD02 4
" , "\
3784674
");

    test_macro!(test5, b"\
DEADBEEFDEADBEEEEEEEEF 16
" , "\
153954073
");

}

// https://atcoder.jp/contests/abc194/tasks/abc194_f
fn _solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let vn = scan.token_bytes()
        .iter().map(|&v| {
            if b'0' <= v && v <= b'9' {
                (v - b'0') as usize
            } else {
                (v - b'A' + 10) as usize
            }
        }).collect::<Vec<_>>();
    let k = scan.token::<usize>();
    let n = vn.len();
    let mut dp = vec![vec![0;k+2];n+1];
    dp[0][0] = 1;
    const MOD:usize = 1000000007;
    for i in 0..n {
        for j in 0..k+1 {
            dp[i+1][j] += dp[i][j] * (k-j);
            dp[i+1][j] %= MOD;
            dp[i+1][j+1] += dp[i][j] * (16-k+(j+1));
            dp[i+1][j+1] %= MOD;
        }
        logln!("{:?}",dp[i+1]);
    }
    let mut ans = 0;
    let mut set = HashSet::new();
    for i in 0..n {
        let d = vn[i];
        let x = set.len();
        if x > k { break; }
        for j in 0..d {
            if i == 0 && j == 0 { continue; }
            if set.contains(&j) {
                ans += dp[n-i-1][k-x];
                ans %= MOD;
            } else {
                if k >= x+1 {
                    ans += dp[n-i-1][k-x-1];
                    ans %= MOD;
                }
            }
        }
        logln!("ans1:{}",ans);
        //if i+2 <= n {
        //    ans += dp[n-i-1][k] + MOD - dp[n-i-2][k-1];
        //    ans %= MOD;
        //}
        logln!("ans2:{}",ans);
        set.insert(d);
    }
    for i in 1..n {
        ans += dp[i-1][k-1] * 15;
        ans %= MOD;
    }
    if set.len() == k {
        ans = (ans+1)%MOD;
    }
    writeln!(out, "{}", ans).ok();
}

// https://atcoder.jp/contests/abc194/tasks/abc194_f
fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let vn = scan.token_bytes()
        .iter().map(|&v| {
            if b'0' <= v && v <= b'9' {
                (v - b'0') as usize
            } else {
                (v - b'A' + 10) as usize
            }
        }).collect::<Vec<_>>();
    let k = scan.token::<usize>();
    let n = vn.len();
    let mut dp = vec![vec![0;k+2];n+1];
    dp[0][0] = 1;
    const MOD:usize = 1000000007;
    for i in 0..n {
        for j in 0..k+1 {
            dp[i+1][j] += dp[i][j] * (k-j);
            dp[i+1][j] %= MOD;
            dp[i+1][j+1] += dp[i][j] * (16-k+(j+1));
            dp[i+1][j+1] %= MOD;
        }
        logln!("{:?}",dp[i+1]);
    }
    let mut ans = 0;
    let mut set = HashSet::new();
    for i in 0..n {
        let d = vn[i];
        if i+2 <= n {
            ans += dp[n-i-1][k] + MOD - dp[n-i-2][k-1];
            ans %= MOD;
        }
        logln!("ans2:{}",ans);
        let x = set.len();
        if x > k { continue; }
        for j in 0..d {
            if i == 0 && j == 0 { continue; }
            if set.contains(&j) {
                ans += dp[n-i-1][k-x];
                ans %= MOD;
            } else {
                if k >= x+1 {
                    ans += dp[n-i-1][k-x-1];
                    ans %= MOD;
                }
            }
        }
        logln!("ans1:{}",ans);
        set.insert(d);
    }
    //for i in 1..n {
    //    ans += dp[i-1][k-1] * 15;
    //    ans %= MOD;
    //}
    if set.len() == k {
        ans = (ans+1)%MOD;
    }
    writeln!(out, "{}", ans).ok();
}
