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
3 2
1 2
1 3
" , "\
2
");

    test_macro!(test2, b"\
4 6
1 2
1 3
1 4
2 3
2 4
3 4
" , "\
1
");

    test_macro!(test3, b"\
10 11
9 10
2 10
8 9
3 4
5 8
1 8
5 6
2 5
3 6
6 9
1 9
" , "\
5
");

    test_macro!(test4, b"\
18 0
" , "\
18
");

}

// https://atcoder.jp/contests/abc222/tasks/abc222_a
fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let m = scan.token::<usize>();
    let mut adj = vec![0;n];
    for _ in 0..m {
        let a = scan.token::<usize>()-1;
        let b = scan.token::<usize>()-1;
        adj[a] |= 1<<b;
        adj[b] |= 1<<a;
    }
    let n2 = 1<<n;
    let inf = 20;
    let mut dp = vec![inf;n2];
    dp[0] = 1;
    for i in 0..n2 { for j in 0..n {
        if i>>j == 0 && i & adj[j] == i && dp[i] == 1{
            dp[i|1<<j] = 1;
        }
    } }
    let mut dp2 = vec![inf;n2];
    for i in 1..n2 {
        let mut j = i;
        while j > 0 {
            let now = dp[i].min(dp2[j]+dp2[i&(!j)]);
            dp2[i] = dp2[i].min(now);
            j=(j-1)&i;
        }
        //logln!("{:b},{}",i,dp2[i]);
    }
    //logln!("{:?}",dp2);
    writeln!(out, "{}",dp2[n2-1]).ok();

}

