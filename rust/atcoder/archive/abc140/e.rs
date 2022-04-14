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
2 3 1
" , "\
5
");

    test_macro!(test2, b"\
5
1 2 3 4 5
" , "\
30
");

    test_macro!(test3, b"\
8
8 2 7 3 4 5 6 1
" , "\
136
");

}

// https://atcoder.jp/contests/abc140/tasks/abc140_e
fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let mut vp = vec![0;n+1];
    for i in 1..n+1 {
        let p = scan.token::<usize>();
        vp[p] = i;
    }
    let mut set = BTreeSet::new();
    let mut ans = 0;
    for i in (1..n+1).rev() {
        let pi = vp[i];
        set.insert(pi);
        let mut range_l = set.range(..pi).copied().rev();
        let l = range_l.next();
        let mut range_r = set.range(pi+1..).copied();
        let r = range_r.next();
        if let Some(l) = l {
            let l2 = range_l.next().unwrap_or(0);
            let r2 = r.unwrap_or(n+1);
            let now = i*(r2-pi)*(l-l2);
            ans += now;
        }
        if let Some(r) = r {
            let r2 = range_r.next().unwrap_or(n+1);
            let l2 = l.unwrap_or(0);
            let now = i*(pi-l2)*(r2-r);
            ans += now;
        }
        logln!("{},{},{}",ans,pi,i);
    }
    writeln!(out, "{}", ans).ok();
}

