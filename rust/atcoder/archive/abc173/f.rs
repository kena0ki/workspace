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

// https://atcoder.jp/contests/abc173/tasks/abc173_f
fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let mut e = vec![Vec::new();n];
    let mut e2 = vec![Vec::new();n];
    for _ in 0..n-1 {
        let mut u = scan.token::<usize>() -1;
        let mut v = scan.token::<usize>() -1;
        if u < v {
            std::mem::swap(&mut u,&mut v);
        }
        e[u].push(v);
        e2[v].push(u);
        logln!("{}->{}", u,v);
    }
    let mut cnts = vec![0;n];
    logln!("{:?}",e);
    cnts[0] = 1;
    for i in 0..n-1 {
        cnts[i+1] = cnts[i] + 1 - e[i+1].len();
    }
    let mut ans = 0;
    let mut sum = cnts.iter().sum::<usize>();
    for i in 0..n {
        logln!("{}", sum);
        ans += sum;
        for &v in &e2[i] {
            sum += n-v;
        }
        sum = sum - (n-i);
    }
    // let mut ans = 0;
    // for i in 0..n {
    //     let mut base = 0;
    //     for j in i..n {
    //         for &v in &e[j] {
    //             if i>0 && v == i-1 {
    //                 base += 1;
    //                 break;
    //             }
    //         }
    //         cnts[j] += base;
    //         if i > 0 {cnts[j] -= 1};
    //         ans += cnts[j];
    //     }
    //     logln!("{:?}", cnts);
    // }
    writeln!(out, "{}", ans).ok();
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
mod abc173f {
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
3
1 3
2 3
" , "\
7
");
    test_macro!(test2, b"\
2
1 2
" , "\
3
");
    test_macro!(test3, b"\
10
5 3
5 7
8 9
1 9
9 10
8 4
7 4
6 10
7 2
" , "\
113
");

}
