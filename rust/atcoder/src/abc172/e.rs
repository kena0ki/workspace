// template

use std::io::{BufRead, BufWriter, Write};
#[allow(unused)]
use std::collections::*;

use rustrithm::math::{combin::Factorial, modulo::ModU64};

fn main() {
    let sin = std::io::stdin();
    let scan = &mut Scanner::new(sin.lock());
    let sout = std::io::stdout();
    let out = &mut BufWriter::new(sout.lock());
    solve(scan, out);
}

// https://atcoder.jp/contests/abc172/tasks/abc172_e
fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let m = scan.token::<usize>();
    const MOD:u64 = 1000000007;
    const ZERO:ModU64<MOD> = ModU64::new(0);
    let fct = Factorial::<MOD>::new(m);
    //let mut ans = fctm.kperm(n);
    let mut ans = ZERO;
    for i in 0..n+1 {
        if i%2 == n%2 {
            ans += fct.combin(n,i) * fct.perm(m-(n-i),i);
        } else {
            ans -= fct.combin(n,i) * fct.perm(m-(n-i),i);
        }
        //logln!("{}",perm);
    }
    ans *= fct.perm(m,n);
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
mod abc172d {
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
2 2
" , "\
2
");
    test_macro!(test2, b"\
2 3
" , "\
18
");
    test_macro!(test3, b"\
141421 356237
" , "\
881613484
");

}
