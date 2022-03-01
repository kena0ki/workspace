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
1
3
53 98
8 43
12 53
","
");

    test_macro!(test1, b"\
5
3
53 98
8 43
12 53
10
4 7
5 7
3 7
4 5
5 8
6 9
4 8
5 10
1 9
5 10
2
58 98
11 29
6
79 83
44 83
38 74
49 88
18 45
64 99
1
5 9
" , "\
Bob
Alice
Bob
Alice
Alice
");

}

// https://atcoder.jp/contests/abc222/tasks/abc222_a
fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let mut t = scan.token::<usize>();
    while t > 0 {
        sub(scan,out);
        t -= 1;
    }
}
fn sub(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    fn f(l:usize, r:usize, lr:&Vec<(usize,usize)>, memo: &mut Vec<Vec<Option<usize>>>) -> usize {
        if memo[l][r].is_some() {
            return memo[l][r].unwrap();
        }
        let mut vg = vec![false;101];
        for &(li,ri) in lr {
            if l <= li && ri <= r {
                let mut x = 0;
                x ^= f(l, li, lr, memo);
                x ^= f(ri, r, lr, memo);
                vg[x] = true;
            }
        }
        let mut g = 0;
        while vg[g] {
            g+=1;
        }
        memo[l][r] = Some(g);
        return g;
    }
    let n = scan.token::<usize>();
    let mut lr = Vec::with_capacity(n);
    for _ in 0..n {
        let l = scan.token::<usize>();
        let r = scan.token::<usize>();
        lr.push((l,r));
    }
    let mut memo = vec![vec![None;100+1];100+1];
    let g = f(1,100,&lr,&mut memo);
    let ans = if g > 0 { "Alice" } else { "Bob" };
    writeln!(out, "{}", ans).ok();
}

