// template

use std::{io::{BufRead, BufWriter, Write}, fmt::Debug};
#[allow(unused)]
use std::collections::*;

use std::ops::{Add, Sub};

#[derive(Debug)]
pub struct FenwickTree<T>{
    identity: T,
    n: usize,
    bit: Vec<T>,
}
impl <T:Debug+Clone+Copy+Add<Output=T>+Sub<Output=T>> FenwickTree<T>{
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
        logln!("r,l:{:?},{:?}",self.sum0(r),self.sum0(l));
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
4 3 2
2 2
3 3
" , "\
10
");

    test_macro!(test2, b"\
5 4 4
3 2
3 4
4 2
5 2
" , "\
14
");

    test_macro!(test3, b"\
200000 200000 0
" , "\
40000000000
");

}

// https://atcoder.jp/contests/abc186/tasks/abc186_f
fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let h = scan.token::<usize>();
    let w = scan.token::<usize>();
    let m = scan.token::<usize>();
    let mut vx = vec![w;h];
    let mut vy = vec![h;w];
    let mut sx = vec![Vec::with_capacity(w);h];
    for _ in 0..m {
        let x = scan.token::<usize>()-1;
        let y = scan.token::<usize>()-1;
        logln!("{},{}",x,y);
        vx[x] = vx[x].min(y);
        vy[y] = vy[y].min(x);
        sx[x].push(y);
    }
    logln!("{:?}",vx);
    logln!("{:?}",vy);
    let mut ans = 0;
    for i in 0..vx[0] {
        ans += vy[i];
    }
    logln!("ans:{}",ans);
    let mut bit = FenwickTree::new(w+1,0);
    for i in vx[0]..w {
        bit.add(i,1);
    }
    for i in 0..vy[0] {
        let x = vx[i];
        ans += bit.sum(0,x);
        for &xi in &sx[i] {
            if bit.sum(xi,xi+1) == 0 {
                bit.add(xi,1);
            }
        }
        logln!("{}: {}",i,ans);
    }
    writeln!(out, "{}",ans).ok();
}

