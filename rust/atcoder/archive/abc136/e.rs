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
2 3
8 20
" , "\
7
");

    test_macro!(test2, b"\
2 10
3 5
" , "\
8
");

    test_macro!(test3, b"\
4 5
10 1 2 22
" , "\
7
");

    test_macro!(test4, b"\
8 7
1 7 5 6 8 2 6 5
" , "\
5
");

}

// https://atcoder.jp/contests/abc136/tasks/abc136_e
fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let k = scan.token::<usize>();
    let mut va = vec![0;n];
    let mut s = 0;
    for i in 0..n {
        let a = scan.token::<usize>();
        va[i] = a;
        s+=a;
    }
    let mut vq = Vec::new();
    let mut cnt = 1;
    while cnt*cnt <= s {
        if s%cnt == 0 {
            vq.push(cnt);
            vq.push(s/cnt);
        }
        cnt+=1;
    }

    let mut ans = 1;
    for &q in &vq {
        let f = |x:usize| {
            if x > s { return false; }
            if s%x > 0 { return false; }
            let mut vm = vec![0;n];
            for i in 0..n {
                vm[i] = va[i]%x;
            }
            vm.sort_unstable();
            let inf = 1<<60;
            let mut c1 = inf;
            for i in 0..n {
                c1=0;
                for j in 0..i {
                    c1 += vm[j];
                }
                let mut c2 = 0;
                for j in i..n {
                    c2 += x-vm[j];

                }
                if c1 == c2 {
                    break;
                }
            }
            logln!("{},{:?},{},{}", x, vm,c1, k);
            assert_ne!(c1,inf);
            return c1 <= k;
        };
        if f(q) {
            ans = ans.max(q);
        }
    }
    writeln!(out, "{}", ans).ok();
}

