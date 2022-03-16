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
3 1
1 10 100
" , "\
3
");

    test_macro!(test1, b"\
3 9
1 5 10
" , "\
3
");

    test_macro!(test2, b"\
5 198
1 5 10 50 100
" , "\
5
");

    test_macro!(test3, b"\
4 44
1 4 20 100
" , "\
4
");

    test_macro!(test4, b"\
9 11837029798
1 942454037 2827362111 19791534777 257289952101 771869856303 3859349281515 30874794252120 216123559764840
" , "\
21
");

}

// https://atcoder.jp/contests/abc182/tasks/abc182_f
fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let x = scan.token::<usize>();
    let mut va = Vec::with_capacity(n);
    for _ in 0..n {
        let a = scan.token::<usize>();
        va.push(a);
    }
    va.push((1<<60)/va[n-1]*va[n-1]);
    logln!("{:?}",va);
    let n = n+1;
    let mut memo = vec![HashMap::with_capacity(n);n];
    let ans = f(x,1,n,&va,0,&mut memo);
    fn f(x:usize,i:usize,n:usize,va:&Vec<usize>,y:usize,memo: &mut Vec<HashMap<usize,Option<usize>>>) -> usize {
        if x == 0 {
            logln!("{},{},{}",x,i,y);
            return 1;
        }
        if i>=n { return 0; }
        if memo[i].contains_key(&x) {
            logln!("{:?}",memo);
            return memo[i][&x].unwrap();
        }
        let mut res = 0;
        let a = va[i];
        if x%a > 0 {
            let nx = x/a*a;
            res += f(nx,i+1,n,va,y+x%a,memo);
        }
        let nx = (x+a-1)/a*a;
        res += f(nx,i+1,n,va,y,memo);
        *memo[i].entry(x).or_default() = Some(res);
        return res;
    }
    writeln!(out, "{}", ans).ok();
}

