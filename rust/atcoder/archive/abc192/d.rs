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
5
4
" , "\
0
");

    test_macro!(test1, b"\
22
10
" , "\
2
");

    test_macro!(test2, b"\
999
1500
" , "\
3
");

    test_macro!(test3, b"\
100000000000000000000000000000000000000000000000000000000000
1000000000000000000
" , "\
1
");

}

// https://atcoder.jp/contests/abc192/tasks/abc192_a
fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let vx = scan.token_bytes().iter().map(|&v| (v-b'0')as usize).collect::<Vec<_>>();
    let m = scan.token::<usize>();
    if vx.len() == 1 {
        if vx[0] <= m {
            writeln!(out, "1").ok();
        } else {
            writeln!(out, "0").ok();
        }
        return;
    }
    let f = |vx:&Vec<usize>, b:usize| {
      let mut res = 0;
      for &x in vx {
          logln!("res:{}",res);
          if res > (m+b-1)/b {
              logln!("OF:{}",(m+b-1)/b);
              return m+1;
          }
          res *= b;
          res += x as usize;
      }
      return res;
    };
    let mut max = 0;
    for &x in &vx {
        max = max.max(x);
    }
    let mut l = max;
    let mut r = m+1;
    logln!("{},{}",l,r);
    while l+1 < r {
        let w = (l+r)/2;
        if f(&vx,w) <= m { l=w; }
        else { r=w; }
    }
    let ans = l - max;
    writeln!(out, "{}",ans).ok();
}

