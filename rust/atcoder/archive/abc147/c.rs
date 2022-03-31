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
3
1
2 1
1
1 1
1
2 0
" , "\
2
");

    test_macro!(test2, b"\
3
2
2 1
3 0
2
3 1
1 0
2
1 1
2 0
" , "\
0
");

    test_macro!(test3, b"\
2
1
2 0
1
1 0
" , "\
1
");

}

// https://atcoder.jp/contests/abc222/tasks/abc222_a
fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let mut vva = vec![Vec::with_capacity(n);n];
    for i in 0..n {
        let m = scan.token::<usize>();
        for _ in 0..m {
            let x = scan.token::<usize>()-1;
            let y = scan.token::<usize>();
            vva[i].push((x,y));
        }
    }
    logln!("{:?}", vva);
    let mut ans = 0;
    let n2 = 1<<n;
    for i in (0..n2).rev() {
        let mut flg = true;
        for j in 0..n {
            if i>>j&1 == 1{
                for &(x,y) in &vva[j] {
                    logln!("{:b},{},{},{}",i,j,x,y);
                    if i>>x&1 == y { continue; }
                    flg=false;
                }
            }
        }
        if flg {
            logln!("{:b}",i);
            ans=ans.max(i.count_ones());
        }
    }
    writeln!(out, "{}", ans).ok();
}

