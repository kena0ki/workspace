// template

use std::{io::{BufRead, BufWriter, Write}, cmp::Reverse};
#[allow(unused)]
use std::collections::*;

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

fn main() {
    let sin = std::io::stdin();
    let scan = &mut Scanner::new(sin.lock());
    let sout = std::io::stdout();
    let out = &mut BufWriter::new(sout.lock());
    solve(scan, out);
}

// https://atcoder.jp/contests/abc203/tasks/abc203_e
fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let k = scan.token::<usize>();
    let m = 20;
    //let m = 3;
    let mut va = vec![(0,0);n];
    for i in 0..n {
        let a = scan.token::<usize>();
        va[i] = (a,i);
    }
    va.sort_unstable_by(|a,b| b.cmp(a));
    logln!("va:{:?}", va);
    let inf = 1usize<<60;
    let mut dp = vec![vec![inf;m+1];n+1];
    dp[0][0] = 0;
    for i in 0..n { for j in 0..m {
        if dp[i][j] < k {
            dp[i+1][j] = dp[i+1][j].min(dp[i][j]+1);
        }
        let ni = va.binary_search_by_key(
            &Reverse((va[i].0/2,usize::max_value())),|&a| Reverse(a));
        let ni = ni.map_or_else(|v| v, |v| v);
        logln!("ni:{}", ni);
        dp[ni][j+1] = dp[ni][j+1].min(dp[i][j]);
        logln!("{:?}", dp);
    }}
    for j in 0..m+1 {
        if dp[n][j] < inf {
            writeln!(out, "{} {}", j, dp[n][j]).ok();
            break;
        }
    }
}

#[allow(unused)]
#[macro_export]
macro_rules! logln {
    ($($arg:tt)*) => ({
        #[cfg(debug_assertions)]
        println!($($arg)*);
    })
}

#[cfg(test)]
mod abc203e {
    use super::*;

    #[test]
    fn test0() {
        let input: &[u8] = b"\
1 1
10
";
        let expected = "\
a a
";
        let output = &mut Vec::new();
        let scan = &mut Scanner::new(input);
        solve(scan, output);

        assert_eq!(expected, std::str::from_utf8(output).unwrap());
    }

    #[test]
    fn test1() {
        let input: &[u8] = b"\
4 1
2 3 4 9
";
        let expected = "\
2 1
";
        let output = &mut Vec::new();
        let scan = &mut Scanner::new(input);
        solve(scan, output);

        assert_eq!(expected, std::str::from_utf8(output).unwrap());
    }
    #[test]
    fn test2() {
        let input: &[u8] = b"\
3 3
2 3 5
";
        let expected = "\
0 3
";
        let output = &mut Vec::new();
        let scan = &mut Scanner::new(input);
        solve(scan, output);

        assert_eq!(expected, std::str::from_utf8(output).unwrap());
    }
    #[test]
    fn test3() {
        let input: &[u8] = b"\
9 8
137 55 56 60 27 28 133 56 55
";
        let expected = "\
1 4
";
        let output = &mut Vec::new();
        let scan = &mut Scanner::new(input);
        solve(scan, output);

        assert_eq!(expected, std::str::from_utf8(output).unwrap());
    }
}
