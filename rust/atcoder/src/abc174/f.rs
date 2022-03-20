// template

use std::io::{BufRead, BufWriter, Write};
#[allow(unused)]
use std::collections::*;

use std::ops::{Add, Sub};

#[derive(Debug)]
pub struct FenwickTree<T>{
    identity: T,
    n: usize,
    bit: Vec<T>,
}
impl <T:Clone+Copy+Add<Output=T>+Sub<Output=T>> FenwickTree<T>{
    pub fn new(n: usize, identity: T) -> FenwickTree<T> {
        return Self {
            identity,
            n,
            bit: vec![identity; n+1],
        };
    }
    /// Adds the value to the given index.
    pub fn add(&mut self, mut idx: usize,a: T){
        if idx >= self.n {
            panic!("Index out of bound. length:{}, but idx:{}.", self.n, idx);
        }
        idx+=1;
        loop {
            if idx > self.n {
                break;
            }
            self.bit[idx] = self.bit[idx]+a;
            let idx64 = idx as i64;
            idx+=(idx64 & -idx64) as usize;
        }
    }
    /// Returns the summary of values between l and r-1.
    pub fn sum(&self, l:usize, r:usize) -> T {
        if l>r {
            panic!("Invalid range. l:{} > r:{}", l, r);
        }
        return self.sum0(r) - self.sum0(l);
    }
    fn sum0(&self, mut idx: usize) -> T {
        //idx+=1;
        let mut ret = self.identity;
        loop {
            if idx<=0 {
                break;
            }
            ret = ret+self.bit[idx];
            let idx64 = idx as i64;
            idx-=(idx64 & -idx64) as usize;
        }
        return ret;
    }
}

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
4 3
1 2 1 3
1 3
2 4
3 3
" , "\
2
3
1
");

    test_macro!(test2, b"\
10 10
2 5 6 5 2 1 7 9 7 2
5 5
2 4
6 7
2 2
7 8
7 9
1 8
6 9
8 10
6 8
" , "\
1
2
2
1
2
2
6
3
3
3
");

}

// https://atcoder.jp/contests/abc174/tasks/abc174_f
fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let q = scan.token::<usize>();
    let mut vvc = vec![Vec::with_capacity(n+1);n+1];
    for i in 1..n+1 {
        let c = scan.token::<usize>();
        vvc[c].push(i);
    }
    let mut tot = 0;
    for i in 0..n+1 {
        if vvc[i].len() > 0 {
            vvc[i].push(n+1);
            tot += 1;
        }
    }
    let mut vvp = vec![Vec::with_capacity(n+1);n+1];
    for i in 1..n+1 {
        let vc = &vvc[i];
        let mut pre = 0;
        for &c in vc {
            vvp[pre].push(c);
            pre = c;
        }
    }
    let mut vvq = vec![Vec::with_capacity(n+1);n+1];
    for i in 0..q {
        let l = scan.token::<usize>();
        let r = scan.token::<usize>();
        vvq[l].push((r,i));
    }
    let mut bit = FenwickTree::new(n+3,0i64);
    let mut ans = vec![tot;q];
    for i in 0..n+1 {
        let vq = &vvq[i];
        for &(r,qi) in vq {
            let s = bit.sum(r+1, n+3);
            ans[qi] -= s;
        }
        let vp = &vvp[i];
        for &y in vp {
            bit.add(y,1);
        }
    }
    for i in 0..q {
        writeln!(out, "{}", ans[i]).ok();
    }
}

