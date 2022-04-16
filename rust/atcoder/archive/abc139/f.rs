// template

use std::io::{BufRead, BufWriter, Write};
#[allow(unused)]
use std::collections::*;
use std::{cmp::Ordering, ops::{Add, Sub}};


#[derive(Debug,Default,Clone,Copy)]
pub struct Vector {
    pub x:f64,
    pub y:f64,
}

impl Vector {
    pub fn new(x: f64, y:f64) -> Self {
        Self { x, y }
    }
    pub fn dot(&self, rhs: &Self) -> f64 {
        return (self.x * rhs.x) + (self.y * rhs.y);
    }
    pub fn cross(&self, rhs: &Self) -> f64 {
        return (self.x * rhs.y) - (self.y * rhs.x);
    }
    pub fn norm2(&self) -> f64 {
        return self.x*self.x + self.y*self.y;
    }
    pub fn norm(&self) -> f64 {
        return ((self.x*self.x + self.y*self.y) as f64).sqrt();
    }
    //pub fn orth(&self) -> i64 {
    //    if self.x == 0 && self.y == 0 { return 0 }
    //    else if self.x > 0 && self.y>=0 { return 1 }
    //    else if self.x <= 0 && self.y>0 { return 2 }
    //    else if self.x < 0 && self.y<=0 { return 3 }
    //    else { return 4 }
    //}
    //pub fn cmp_angle(&self, rhs: &Self) -> Ordering {
    //    let o1 = self.orth();
    //    let o2 = rhs.orth();
    //    if o1 != o2 {
    //        return o1.cmp(&o2);
    //    }
    //    let c = self.cross(rhs);
    //    return 0.cmp(&c);
    //}
    pub fn orth(&self,eps:f64) -> i64 {
        if self.x.abs() < eps && self.y.abs() < eps { return 0 }
        else if self.x > 0f64 && self.y>=0f64 { return 1 }
        else if self.x <= 0f64 && self.y>0f64 { return 2 }
        else if self.x < 0f64 && self.y<=0f64 { return 3 }
        else { return 4 }
    }
    pub fn partial_cmp_angle(&self, rhs: &Self, eps:f64) -> Option<Ordering> {
        let o1 = self.orth(eps);
        let o2 = rhs.orth(eps);
        if o1 != o2 {
            return Some(o1.cmp(&o2));
        }
        let c = self.cross(rhs);
        return 0f64.partial_cmp(&c);
    }
}

impl Add for Vector {
    type Output=Self;
    fn add(self, rhs: Self) -> Self {
        Self { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}
impl Sub for Vector {
    type Output=Self;
    fn sub(self, rhs: Self) -> Self {
        Self { x: self.x - rhs.x, y: self.y - rhs.y }
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
3
0 10
5 -5
-5 -5
" , "\
10.000000000000000000000000000000000000000000000000
");

    test_macro!(test2, b"\
5
1 1
1 0
0 1
-1 0
0 -1
" , "\
2.828427124746190097603377448419396157139343750753
");

    test_macro!(test3, b"\
5
1 1
2 2
3 3
4 4
5 5
" , "\
21.213203435596425732025330863145471178545078130654
");

    test_macro!(test4, b"\
3
0 0
0 1
1 0
" , "\
1.414213562373095048801688724209698078569671875376
");

}

// https://atcoder.jp/contests/abc139/tasks/abc139_f
fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let mut vv = Vec::with_capacity(n);
    for _ in 0..n {
        let x = scan.token::<f64>();
        let y = scan.token::<f64>();
        vv.push(Vector::new(x,y));
    }
    vv.sort_unstable_by(|a,b| a.partial_cmp_angle(b,0.1).unwrap());
    //vv.sort_unstable_by(|a,b| a.cmp_angle(b));
    let mut ans = 0f64;
    for i in 0..n {
        let mut p = Vector::default();
        for j in 0..n {
            p = p.add(vv[(i+j)%n]);
            ans = ans.max(p.norm2() as f64);
        }
    }
    let ans = ans.sqrt();
    writeln!(out, "{}", ans).ok();
}

