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
5 3
10 14 19 34 33
" , "\
202
");

    test_macro!(test2, b"\
9 14
1 3 5 110 24 21 34 5 3
" , "\
1837
");

    test_macro!(test3, b"\
9 73
67597 52981 5828 66249 75177 64141 40773 79105 16076
" , "\
8128170
");

}

// https://atcoder.jp/contests/abc149/tasks/abc149_e
fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let m = scan.token::<usize>();
    let mut va = vec![(0,0);n];
    for i in 0..n {
        let a = scan.token::<i64>();
        va[i] = (a,i);
    }
    va.sort_unstable_by(|a,b| b.cmp(a));
    let mut vas = vec![0;n+1];
    for i in 0..n {
        vas[i+1] = va[i].0 + vas[i];
    }
    let f = |x:i64| {
        let mut cnt = 0;
        let mut tot = 0;
        for i in 0..n {
            let y = x-va[i].0;
            let j = va.binary_search_by(|prob| (y-1,usize::max_value()).cmp(prob))
                .map_or_else(|v| v,|v| v);
            if j <= 0 { break; }
            cnt += j;
            tot += vas[j]+va[i].0*j as i64;
            //logln!("{}:{},{},{},{},{},{}", i,x,y,j,tot,cnt,va[j-1].0+va[i].0);
        }
        return (cnt,tot);
    };
    let mut l = 0;
    let mut r = 1000_000_000_000;
    while l+1<r {
        let x = (l+r)/2;
        let ret = f(x);
        if ret.0 >= m { l=x; }
        else { r=x; }
    }
    let res = f(l);
    let ans = res.1 - ((res.0-m) as i64 *l);
    writeln!(out, "{}", ans).ok();
}

