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
3 1
2 2 1
" , "\
4
");

    test_macro!(test2, b"\
5 2
3 3 2
4 4 3
" , "\
90
");

    test_macro!(test3, b"\
18 0
" , "\
6402373705728000
");

}

// https://atcoder.jp/contests/abc199/tasks/abc199_e
fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let m = scan.token::<usize>();
    let mut va = vec![Vec::with_capacity(m);n+1];
    for _ in 0..m {
        let x = scan.token::<usize>();
        let y = scan.token::<usize>();
        let z = scan.token::<usize>();
        va[x].push((y,z));
    }
    let n2 = 1<<n;
    let mut dp = vec![0usize;n2];
    dp[0] = 1;
    for i in 0..n2 {
        let x = i.count_ones() as usize;
        let mut ok = true;
        for &(y,z) in &va[x] {
            let i2 = i>>(n-y);
            //logln!("i2:{},{:b}", i2,i2);
            if i2.count_ones() as usize > z {
                ok = false;
                break;
            }
        }
        if ok {
            //logln!("i:{},{:b}", i,i);
            for j in 0..n {
                if i>>j&1 == 0 {
                    dp[i|1<<j] += dp[i];
                }
            }
        }
        //logln!("{:?}",dp);
    }
    writeln!(out, "{}", dp[n2-1]).ok();
}

