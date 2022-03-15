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
5 17
2 3 5 7 11
" , "\
17
");

    test_macro!(test2, b"\
6 100
1 2 7 5 8 10
" , "\
33
");

    test_macro!(test3, b"\
6 100
101 102 103 104 105 106
" , "\
0
");

    test_macro!(test4, b"\
7 273599681
6706927 91566569 89131517 71069699 75200339 98298649 92857057
" , "\
273555143
");

}

// https://atcoder.jp/contests/abc184/tasks/abc184_f
fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let t = scan.token::<usize>();
    let m = n/2;
    let m2 = 1<<m;
    let mut vx = Vec::<usize>::with_capacity(m2+1);
    let mut vy = Vec::<usize>::with_capacity(m2+1);
    vx.push(0);
    vy.push(0);
    let mut f = |va: &mut Vec<usize>,c:usize| {
        for _ in 0..c {
            let a = scan.token::<usize>();
            let len = va.len();
            for i in 0..len {
                va.push(va[i]+a);
            }
        }
    };
    f(&mut vx, m);
    f(&mut vy, n-m);
    vx.sort_unstable();
    vy.sort_unstable();
    logln!("{:?},{}",vx, vx.len());
    logln!("{:?},{}",vy, vy.len());
    //let mut tmp=0;
    //for i in 0..vx.len() {
    //    for j in 0..vy.len() {
    //        let sum = vx[i]+vy[j];
    //        if sum <= t {
    //            tmp = tmp.max(sum);
    //        }
    //    }
    //}
    //logln!("tmp:{}",tmp);
    let mut i = 0;
    let mut j = vy.len()-1;
    let mut ans = 0;
    while i<m2 || j > 0 {
        let sum = vx[i] + vy[j];
        logln!("{}+{}={}",vx[i],vy[j],sum);
        if sum <= t {
            ans = ans.max(sum);
            if i+1 == vx.len() { break; }
            i+=1;
        } else {
            if j==0 { break; }
            j-=1;
        }
    }
    writeln!(out, "{}", ans).ok();
}

